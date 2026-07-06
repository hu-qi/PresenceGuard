# Presence Guard — Rust-first macOS implementation

Presence Guard is a menu-bar app that uses the Mac camera locally to verify the enrolled user is still present. It hides screen content when an unknown person or several people are detected, and locks the macOS session when the user has left or an unknown face remains visible.

This repository is **Rust-first**:

- `presence-core` is pure Rust and contains the reusable policy state machine, event contract, and tests.
- `presenceguard-macos` is the Rust application runtime. It owns encryption, transactional enrollment persistence, Keychain orchestration, audit logs, status transitions, policy actions, and macOS lock execution.
- `macos/PGBridge.m` is intentionally a narrow native adapter only. It binds AVFoundation, Vision, AppKit, and Keychain to macOS because these OS frameworks are not cross-platform concerns. It does not decide lock policy or persist biometric material.

The Windows and HarmonyOS implementations can retain `presence-core` and replace only the platform adapter that produces `FaceSignal` events and executes `ProtectionAction` values.

## Actual behavior

- Real AVFoundation camera capture at 640×480; camera frames are processed in memory and never written to disk.
- Real Vision face detection, capture-quality checks, yaw/pitch checks, face cropping, and `VNFeaturePrintObservation` distance matching.
- Real 12-sample enrollment flow.
- Rust AES-256-GCM encryption of every persisted enrollment sample.
- A random 32-byte key stored in the macOS Keychain as `kSecAttrAccessibleWhenUnlockedThisDeviceOnly`.
- Rust-owned transactional persistence at `~/Library/Application Support/io.tarkwong.presenceguard/enrollment`.
- Real macOS menu-bar application, multi-display privacy shield, and macOS session lock through Apple’s installed `CGSession -suspend` command.
- Local audit log at `~/Library/Application Support/io.tarkwong.presenceguard/audit.log`; it records events only, not images, templates, or face distances.
- No network calls, cloud API, mock data, or simulated camera path.

## Build on macOS

Requirements:

- macOS 14 or newer
- Xcode Command Line Tools
- Rust stable toolchain
- A camera available to macOS

```bash
./scripts/build-app.sh
open dist/PresenceGuard.app
```

The app lives in the menu bar. On first launch, grant Camera permission. Choose **Enroll authorized face** and keep one face in view until all 12 samples are captured. Automatic protection is enabled after a successful enrollment.

For an application installation under your user account:

```bash
./scripts/install-app.sh
```

## Security model

The app does not read or reuse the operating system’s Face ID / Touch ID / Windows Hello template. Enrollment is application-specific. Original camera frames are never persisted. Only selected face crops from the explicit enrollment flow are encrypted locally; the encryption key is non-exportable through normal application storage and remains in the macOS Keychain.

This is a continuous privacy-protection application, not a replacement for FileVault, macOS login, MDM, or certified liveness authentication.

## Cross-platform boundary

```text
Camera / face engine → FaceSignal ┐
                                 ├── presence-core (Rust) → ProtectionAction → platform executor
System lock / privacy shield ────┘
```

The Rust core does not know whether a signal comes from Apple Vision, Windows Media Foundation + ONNX Runtime, or HarmonyOS Camera Kit + NDK. That is the intended cross-platform seam.
