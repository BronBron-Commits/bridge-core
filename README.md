# bridge-core

**bridge-core** is a portable, stateful native core implemented in Rust, exposed through a stable C ABI, and driven by managed or native hosts (currently validated via .NET on Android-class Linux / Termux).

It is designed to be a **single core engine** that can be reused unchanged across:

- Desktop (Linux, Windows, macOS)
- Mobile (Android, iOS)
- Managed runtimes (.NET, JVM, etc.)
- Other native hosts (C/C++, WASM loaders)

The core owns all state. Hosts interact with it through an opaque handle and a command-based ABI.

## What This Repository Demonstrates

This repository is not a demo scaffold. It proves, end-to-end:

- A Rust core compiled as an Android-compatible `cdylib`
- Explicit symbol exports that survive Android/LLVM dead-code stripping
- A single, extensible command entry point
- ABI versioning and validation
- Panic containment (no Rust panics cross the FFI boundary)
- Successful invocation from .NET via P/Invoke
- Correct state mutation across multiple native calls

All of this has been validated on Termux (Android / aarch64).

## High-Level Architecture

<!-- If you have an architecture diagram (e.g., architecture.png), add it here: -->
<!-- ![High-Level Architecture](architecture.png) -->

*(Diagram or further details can be added here in the future.)*
