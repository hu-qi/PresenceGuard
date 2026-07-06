# Presence Guard

> **Local, Rust-first presence protection for desktop devices.**
>
> Detect when the enrolled user is no longer at the screen, protect visible content from unknown or multiple viewers, and request a system lock through a platform adapter.

[中文文档](README.zh-CN.md) · [Architecture](docs/architecture.md) · [Contributing](CONTRIBUTING.md) · [Security](SECURITY.md) · [Release process](RELEASING.md)

## Why Presence Guard

Operating-system idle timers cannot distinguish between an authorized user, an unknown viewer, and an empty seat. Presence Guard provides a reusable decision layer for local, privacy-oriented protection:

- **Authorized user present**: keep the screen available.
- **No face for a configured duration**: request a system lock.
- **Unknown face for consecutive samples**: show a privacy shield, then request a system lock.
- **Multiple faces**: show a privacy shield without treating the event as a failed identity match.
- **Unreliable camera signal**: report an unavailable state instead of incorrectly locking the session.

The project is designed for local processing. It does not use cloud identity services or upload camera frames.

## Project status

| Component | Status | Scope |
| --- | --- | --- |
| `presence-core` | Implemented | Pure Rust policy state machine and unit tests. |
| macOS adapter | Import validation in progress | AVFoundation/Vision/AppKit bridge, menu-bar UI, privacy shield, and system-lock executor. |
| Windows adapter | Planned | Platform integration only; policy remains in Rust core. |
| HarmonyOS PC adapter | Planned | Platform integration only; policy remains in Rust core. |

The macOS source import is intentionally verified by GitHub Actions before it is committed to the standard source tree. The workflow validates the staged archive checksum, rejects unexpected archive paths, builds on a macOS runner, and removes the temporary import payload only after validation succeeds.

## Architecture

```text
Camera / face engine ──> FaceSignal ┐
                                    ├── presence-core (Rust) ──> ProtectionAction ──> platform executor
System lock / privacy shield <──────┘
```

`presence-core` owns the state machine, debounce logic, confidence-event handling, and protection decisions. Each desktop platform supplies only two adapters:

1. a source of normalized identity/presence signals; and
2. an executor for privacy shielding and system-session locking.

This separation lets Windows, macOS, and HarmonyOS PC share the same policy semantics without forcing them to share camera or system APIs.

## Repository layout

```text
.
├── crates/
│   └── presence-core/          # Cross-platform Rust policy crate
├── docs/                       # Architecture and project documentation
├── .github/workflows/          # CI, releases, and verified source-import workflow
├── CONTRIBUTING.md
├── SECURITY.md
├── RELEASING.md
└── README.zh-CN.md
```

## Quick start

### Prerequisites

- Rust stable toolchain
- Git

### Validate the Rust core

```bash
cargo fmt --all -- --check
cargo clippy -p presence-core --all-targets -- -D warnings
cargo test -p presence-core --all-targets
```

The macOS app requires macOS, Xcode Command Line Tools, a camera, and the platform adapter source. It is not represented as a cross-platform binary crate.

## Privacy and safety

- The project is intended to process presence and face-comparison signals locally.
- It must not be treated as a replacement for operating-system login, FileVault, endpoint management, or certified liveness authentication.
- An uncertain or unavailable camera signal must not be interpreted as proof that an unknown person is present.
- Product deployments that persist biometric material must obtain the appropriate consent and meet the applicable privacy and security requirements.

See [SECURITY.md](SECURITY.md) for vulnerability reporting and security boundaries.

## CI and releases

- **CI** runs formatting, Clippy, tests, and a core build on Linux, macOS, and Windows runners.
- **Release** runs when a tag matching `vMAJOR.MINOR.PATCH` is pushed. It validates the Rust core, publishes the packaged `presence-core` crate as a GitHub Release asset, and adds `SHA256SUMS.txt`.
- **macOS source import** is a one-time verified import pipeline for source that cannot be written directly through the remote integration.

See [RELEASING.md](RELEASING.md) for the exact release commands and release criteria.

## Contributing

Contributions are welcome. Start with [CONTRIBUTING.md](CONTRIBUTING.md), keep platform-specific integration behind the adapter boundary, and do not add biometric samples, tokens, build products, or private logs to commits.

## License

MIT. See [LICENSE](LICENSE).
