# Contributing

Thanks for helping improve Presence Guard.

## Principles

- Keep `presence-core` independent of operating-system APIs, camera APIs, UI frameworks, and network services.
- Put platform-specific behavior behind adapters.
- Do not commit local user data, captured images, identity data, credentials, signing material, build output, or audit logs.
- Treat unavailable or low-quality camera input as uncertain. It is not proof of an unknown viewer.

## Before opening a pull request

Run:

```bash
cargo fmt --all -- --check
cargo clippy -p presence-core --all-targets -- -D warnings
cargo test -p presence-core --all-targets
```

Describe the behavior change, relevant privacy impact, and test evidence in the pull request.

## Core and adapter boundary

`presence-core` converts normalized `FaceSignal` values into `ProtectionAction` values. A platform adapter obtains local signals and applies actions such as screen shielding or system locking.

A policy change must state expected behavior for an authorized user, no face, unknown face, multiple faces, and unavailable input.

## Commit style

Use focused imperative messages:

```text
feat(core): add unknown-face debounce
fix(macos): stop camera when paused
docs: clarify privacy boundary
```

For potential security or privacy issues, use the private reporting route described in [SECURITY.md](SECURITY.md) rather than a public issue.
