# Security Policy

## Supported versions

Only the current `main` branch is supported while the project is pre-release.

## Reporting a vulnerability

Please do not disclose suspected security or privacy vulnerabilities in public issues.

Use GitHub's private vulnerability reporting feature for this repository. Include:

- affected revision or commit;
- a clear description of the security or privacy impact;
- steps to reproduce without sharing real personal data;
- a suggested mitigation when available.

You should receive an acknowledgement after the report is reviewed. Public disclosure should wait until a fix or mitigation is available.

## Security boundaries

Presence Guard is a local protection aid. It is not a replacement for:

- operating-system authentication;
- full-disk encryption;
- device-management controls;
- a certified liveness or anti-spoofing system.

The project should avoid storing raw camera frames. Any persisted identity material must have a documented purpose, local protection mechanism, deletion path, and explicit user consent appropriate to the deployment environment.

## Out of scope

Reports about unsupported hardware, missing product features, or false matches without a reproducible security impact should be filed as normal issues after removing personal data.
