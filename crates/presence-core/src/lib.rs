//! Platform-independent policy for Presence Guard.
//! Platform adapters produce [`FaceSignal`] values and execute [`ProtectionAction`] values.

use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtectionConfig {
    pub absence_lock_after: Duration,
    pub unknown_lock_after: Duration,
    pub unknown_confirmation_samples: u32,
    pub shield_on_multiple_faces: bool,
}

impl Default for ProtectionConfig {
    fn default() -> Self {
        Self {
            absence_lock_after: Duration::from_secs(15),
            unknown_lock_after: Duration::from_secs(5),
            unknown_confirmation_samples: 3,
            shield_on_multiple_faces: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FaceSignal {
    Authorized { distance: f32 },
    Unknown { distance: f32 },
    NoFace,
    MultipleFaces { count: u32 },
    Unreliable { reason: String },
    Unavailable { reason: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProtectionStatus {
    Inactive,
    NeedsEnrollment,
    Starting,
    Authorized { distance: f32 },
    VerifyingUnknown { accepted_samples: u32, required_samples: u32 },
    ShieldingUnknown { seconds_remaining: u64 },
    Away { seconds_remaining: u64 },
    ShieldingMultipleFaces { count: u32 },
    CameraUnavailable { reason: String },
    Locking { reason: String },
}

impl ProtectionStatus {
    pub fn title(&self) -> &'static str {
        match self {
            Self::Inactive => "Protection paused",
            Self::NeedsEnrollment => "Face enrollment required",
            Self::Starting => "Starting protection services",
            Self::Authorized { .. } => "Authorized user verified",
            Self::VerifyingUnknown { .. } => "Verifying a new face",
            Self::ShieldingUnknown { .. } => "Unknown face detected",
            Self::Away { .. } => "User appears away",
            Self::ShieldingMultipleFaces { .. } => "Multiple faces detected",
            Self::CameraUnavailable { .. } => "Camera unavailable",
            Self::Locking { .. } => "Locking device",
        }
    }

    pub fn detail(&self) -> String {
        match self {
            Self::Inactive => "Camera monitoring is paused.".into(),
            Self::NeedsEnrollment => "Enroll an authorized face before enabling protection.".into(),
            Self::Starting => "Preparing local camera and identity checks.".into(),
            Self::Authorized { distance } => format!("Local face match confirmed (distance {distance:.3})."),
            Self::VerifyingUnknown { accepted_samples, required_samples } => {
                format!("Collecting confirmation sample {accepted_samples}/{required_samples} before protection.")
            }
            Self::ShieldingUnknown { seconds_remaining } => {
                format!("Privacy shield is active. Locking in {seconds_remaining}s unless the enrolled user returns.")
            }
            Self::Away { seconds_remaining } => {
                format!("No face detected. Locking in {seconds_remaining}s unless the enrolled user returns.")
            }
            Self::ShieldingMultipleFaces { count } => {
                format!("Privacy shield is active while {count} faces are visible.")
            }
            Self::CameraUnavailable { reason } => reason.clone(),
            Self::Locking { reason } => reason.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProtectionAction {
    SetStatus(ProtectionStatus),
    ShowShield { message: String },
    HideShield,
    Lock { reason: String },
}

#[derive(Debug, Default)]
pub struct ProtectionPolicy {
    absence_started_at: Option<Instant>,
    unknown_started_at: Option<Instant>,
    unknown_samples: u32,
    shield_visible: bool,
    locking: bool,
}

impl ProtectionPolicy {
    pub fn reset(&mut self) -> Vec<ProtectionAction> {
        self.absence_started_at = None;
        self.unknown_started_at = None;
        self.unknown_samples = 0;
        self.locking = false;
        let mut actions = Vec::new();
        if self.shield_visible {
            self.shield_visible = false;
            actions.push(ProtectionAction::HideShield);
        }
        actions.push(ProtectionAction::SetStatus(ProtectionStatus::Inactive));
        actions
    }

    pub fn consume(&mut self, signal: FaceSignal, config: ProtectionConfig, now: Instant) -> Vec<ProtectionAction> {
        if self.locking {
            return vec![ProtectionAction::SetStatus(ProtectionStatus::Locking {
                reason: "Waiting for macOS to lock the current session.".into(),
            })];
        }
        match signal {
            FaceSignal::Authorized { distance } => {
                self.absence_started_at = None;
                self.unknown_started_at = None;
                self.unknown_samples = 0;
                let mut actions = Vec::new();
                if self.shield_visible {
                    self.shield_visible = false;
                    actions.push(ProtectionAction::HideShield);
                }
                actions.push(ProtectionAction::SetStatus(ProtectionStatus::Authorized { distance }));
                actions
            }
            FaceSignal::NoFace => {
                self.unknown_started_at = None;
                self.unknown_samples = 0;
                let started = self.absence_started_at.get_or_insert(now);
                let elapsed = now.saturating_duration_since(*started);
                if elapsed >= config.absence_lock_after {
                    self.locking = true;
                    return vec![ProtectionAction::Lock {
                        reason: format!("No face was detected for {} seconds.", config.absence_lock_after.as_secs()),
                    }];
                }
                vec![ProtectionAction::SetStatus(ProtectionStatus::Away {
                    seconds_remaining: config.absence_lock_after.as_secs().saturating_sub(elapsed.as_secs()),
                })]
            }
            FaceSignal::Unknown { .. } => {
                self.absence_started_at = None;
                self.unknown_samples = self.unknown_samples.saturating_add(1);
                if self.unknown_samples < config.unknown_confirmation_samples {
                    return vec![ProtectionAction::SetStatus(ProtectionStatus::VerifyingUnknown {
                        accepted_samples: self.unknown_samples,
                        required_samples: config.unknown_confirmation_samples,
                    })];
                }
                let started = self.unknown_started_at.get_or_insert(now);
                let elapsed = now.saturating_duration_since(*started);
                let mut actions = Vec::new();
                if !self.shield_visible {
                    self.shield_visible = true;
                    actions.push(ProtectionAction::ShowShield {
                        message: "Unknown person detected. Screen content is protected until the enrolled user returns.".into(),
                    });
                }
                if elapsed >= config.unknown_lock_after {
                    self.locking = true;
                    actions.push(ProtectionAction::Lock {
                        reason: format!("An unknown face remained visible for {} seconds.", config.unknown_lock_after.as_secs()),
                    });
                } else {
                    actions.push(ProtectionAction::SetStatus(ProtectionStatus::ShieldingUnknown {
                        seconds_remaining: config.unknown_lock_after.as_secs().saturating_sub(elapsed.as_secs()),
                    }));
                }
                actions
            }
            FaceSignal::MultipleFaces { count } => {
                self.absence_started_at = None;
                self.unknown_started_at = None;
                self.unknown_samples = 0;
                if !config.shield_on_multiple_faces {
                    return vec![ProtectionAction::SetStatus(ProtectionStatus::VerifyingUnknown {
                        accepted_samples: 0,
                        required_samples: config.unknown_confirmation_samples,
                    })];
                }
                let mut actions = Vec::new();
                if !self.shield_visible {
                    self.shield_visible = true;
                    actions.push(ProtectionAction::ShowShield {
                        message: "Multiple faces detected. Screen content is temporarily protected.".into(),
                    });
                }
                actions.push(ProtectionAction::SetStatus(ProtectionStatus::ShieldingMultipleFaces { count }));
                actions
            }
            FaceSignal::Unreliable { reason } | FaceSignal::Unavailable { reason } => {
                vec![ProtectionAction::SetStatus(ProtectionStatus::CameraUnavailable { reason })]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authorized_face_hides_privacy_shield() {
        let config = ProtectionConfig::default();
        let start = Instant::now();
        let mut policy = ProtectionPolicy::default();
        for offset in 0..3 {
            policy.consume(FaceSignal::Unknown { distance: 0.9 }, config, start + Duration::from_secs(offset));
        }
        let actions = policy.consume(FaceSignal::Authorized { distance: 0.1 }, config, start + Duration::from_secs(3));
        assert!(actions.iter().any(|item| matches!(item, ProtectionAction::HideShield)));
    }

    #[test]
    fn absence_eventually_locks() {
        let config = ProtectionConfig { absence_lock_after: Duration::from_secs(2), ..ProtectionConfig::default() };
        let start = Instant::now();
        let mut policy = ProtectionPolicy::default();
        policy.consume(FaceSignal::NoFace, config, start);
        let actions = policy.consume(FaceSignal::NoFace, config, start + Duration::from_secs(2));
        assert!(actions.iter().any(|item| matches!(item, ProtectionAction::Lock { .. })));
    }
}
