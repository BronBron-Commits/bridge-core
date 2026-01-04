Architecture

This repository implements a single-core / multi-host architecture.

The Rust core is the source of truth. All hosts (managed or native)
interact with it through a stable C ABI.

The design explicitly avoids:
- shared memory across languages
- language-specific data structures at the boundary
- callbacks during early evolution

Core Principles

1. Rust Owns All State
The host never allocates or mutates Rust memory directly.
All state lives behind an opaque BridgeContext.

2. Opaque Handles Only
The ABI exposes pointers but not structure layouts.
This guarantees ABI stability over time.

3. Command-Based Dispatch
Instead of many exported functions, a single dispatcher is used.
This allows the API to grow without breaking loaders or bindings.

4. Explicit Failure
Errors are returned as codes.
Panics are caught and never cross the FFI boundary.

Data Flow

Host:
- creates context
- sends commands
- receives primitive outputs

Core:
- mutates internal state
- returns results
- never calls back into host

Why This Matters

This architecture survives:
- language changes
- platform changes
- mobile sandboxing
- long-term ABI evolution

It is the same pattern used by:
- database engines
- graphics APIs
- simulation cores
- protocol runtimes
