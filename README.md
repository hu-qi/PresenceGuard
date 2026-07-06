# Presence Guard

A Rust-first foundation for a local, cross-platform presence-protection application.

The repository currently contains the reusable protection-policy core. It decides when a platform adapter should hide screen content or lock the current session based on these signals:

- authorized face present;
- unknown face present for several consecutive samples;
- no face detected for a configured duration;
- multiple faces detected;
- camera or image-quality failure.

## Current repository state

`presence-core` is implemented and committed. It is pure Rust, contains the state machine and unit tests, and has no operating-system or camera dependency.

The macOS adapter source is **not yet present in this GitHub repository**. It requires local macOS compilation and contains the AVFoundation, Vision, AppKit, and system-lock integrations. The earlier local source archive remains separate from this repository until it can be reviewed and uploaded through an allowed write path.

No camera, facial image, feature vector, Keychain item, encryption key, build output, or local audit log has been committed to GitHub.

## Architecture

```text
Camera / face engine → FaceSignal ┐
                                 ├── presence-core (Rust) → ProtectionAction → platform executor
System lock / privacy shield ────┘
```

The intended platform boundary is:

- Windows: camera and system-session adapter.
- macOS: AVFoundation/Vision and AppKit/session-lock adapter.
- HarmonyOS PC: Camera Kit/NDK and system-session adapter.

The Rust core deliberately does not depend on any one operating-system API.

## Development

The core can be tested once a Rust toolchain is available:

```bash
cargo test -p presence-core
```

This project is a privacy-protection aid, not a replacement for macOS login, FileVault, MDM, or certified liveness authentication.
