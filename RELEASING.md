# Releasing Presence Guard

## Release policy

Releases are created from annotated semantic-version tags:

```text
vMAJOR.MINOR.PATCH
```

Examples: `v0.1.0`, `v0.1.1`, `v1.0.0-rc.1`.

The release workflow validates tag format, checks formatting, runs Clippy and unit tests for `presence-core`, packages the Rust crate, generates `SHA256SUMS.txt`, and creates or updates the GitHub Release.

## Preconditions

Before creating a tag:

1. `main` is green in CI.
2. The changelog has an entry for the version.
3. The version in the workspace manifest is updated when required.
4. Release notes accurately describe behavior and privacy-impacting changes.
5. No local user data, captured images, identity data, credentials, or build output is included in the release commit.

## Create a release

```bash
git checkout main
git pull --ff-only
git tag -a v0.1.0 -m "Presence Guard v0.1.0"
git push origin v0.1.0
```

GitHub Actions publishes the source package and checksum file to the matching GitHub Release.

## Manual re-run

The Release workflow also supports `workflow_dispatch`. Provide an existing tag in the `tag` input. The workflow does not create tags; it only verifies and publishes the named tag.

## Release assets

The current release pipeline publishes:

- `presence-core-<version>.crate`
- `SHA256SUMS.txt`

A signed or notarized macOS application must not be published as a release asset until the macOS adapter has passed macOS CI and the signing/notarization policy is documented.
