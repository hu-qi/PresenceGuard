# Architecture

## Goal

Presence Guard separates protection policy from operating-system integration.

## Flow

```text
local camera or presence engine
        ↓
normalized FaceSignal
        ↓
presence-core
        ↓
ProtectionAction
        ↓
platform shield and platform lock operation
```

## Core signals

`presence-core` handles these input categories:

- `Authorized`: the enrolled user is present;
- `Unknown`: a detected face does not match the enrolled user;
- `NoFace`: no usable face is visible;
- `MultipleFaces`: more than one face is visible;
- `Unreliable` or `Unavailable`: capture or quality information cannot support a decision.

## Core actions

The core returns only platform-neutral actions:

- update visible status;
- show a privacy shield;
- hide a privacy shield;
- request system locking.

## Adapter contract

An adapter may use native camera and system APIs, but it must:

1. request the required platform permission;
2. normalize raw results into the core signal model;
3. execute actions through supported system facilities;
4. avoid uploading camera material by default;
5. preserve the distinction between uncertain input and an unknown user.

## Failure handling

The intended policy is conservative about identity claims and explicit about screen protection. Unknown-viewer detection requires consecutive evidence. Missing or unreliable camera data produces a visible unavailable state rather than an automatic identity conclusion.
