
bridge-core

bridge-core is a portable, stateful native core implemented in Rust, exposed through a stable C ABI, and driven by managed or native hosts (currently validated via .NET on Android-class Linux / Termux).

It is designed to be a single core engine that can be reused unchanged across:

Desktop (Linux / Windows / macOS)

Mobile (Android, iOS)

Managed runtimes (.NET, JVM, etc.)

Other native hosts (C/C++, WASM loaders)


The core owns all state. Hosts interact with it through an opaque handle and a command-based ABI.


---

What This Repository Demonstrates

This repository is not a demo scaffold. It proves, end-to-end:

A Rust core compiled as an Android-compatible cdylib

Explicit symbol exports that survive Android/LLVM dead-stripping

A single, extensible command entry point

ABI versioning and validation

Panic containment (no Rust panics cross FFI)

Successful invocation from .NET via P/Invoke

Correct state mutation across multiple native calls


All of this has been validated on Termux (Android / aarch64).


---

High-Level Architecture

┌──────────────┐
│  Host (C#)   │  .NET / other languages
│              │
│  - Creates   │
│  - Sends     │
│  - Queries   │
└──────┬───────┘
       │ C ABI (opaque pointers + commands)
       ▼
┌──────────────────────┐
│   bridge-core (Rust) │
│                      │
│  - Owns all state    │
│  - Executes commands │
│  - Returns results   │
│  - Never panics out  │
└──────────────────────┘

The host never accesses Rust memory directly.
All interaction goes through bridge_command.


---

Core Concepts

Opaque Context

typedef struct BridgeContext BridgeContext;

The host receives a BridgeContext* but cannot inspect it.
This guarantees ABI stability and prevents cross-language memory bugs.


---

Command-Based ABI

Instead of exporting many functions, the core exposes one command dispatcher:

int32_t bridge_command(
    BridgeContext* ctx,
    uint32_t cmd,
    int64_t arg,
    int64_t* out
);

This allows the API to grow without adding new symbols, which is critical for long-term compatibility.


---

ABI Versioning

uint32_t bridge_abi_version(void);

Hosts can refuse to load incompatible versions instead of failing silently.


---

Panic Safety

All native entry points are wrapped with catch_unwind.
If Rust panics internally, the host receives an error code instead of crashing.


---

Current Commands

Command	ID	Description

Step	1	Advances an internal tick counter
Reset	2	Resets all internal state
GetLast	3	Returns the last computed result
Add	4	Adds a value to internal state


These are intentionally simple; they exist to prove correctness of the ABI and state handling.


---

Repository Layout

bridge-core/
├─ rust/
│  └─ core/
│     ├─ src/lib.rs        # Rust core + FFI boundary
│     ├─ include/          # Public C header
│     └─ Cargo.toml
├─ dotnet/
│  └─ Bridge.Host/         # .NET test host (P/Invoke)
├─ scripts/                # Reserved for tooling
└─ README.md


---

Building the Core (Android / Termux)

cd rust/core
cargo build --target aarch64-linux-android

This produces:

target/aarch64-linux-android/debug/libbridge_core.so


---

Running the .NET Host (Validated Path)

export LD_LIBRARY_PATH=$PWD/rust/core/target/aarch64-linux-android/debug
cd dotnet/Bridge.Host
dotnet run

Expected output:

Managed entry reached
ABI=1
Context created
add1=2
add2=5
tick=1
last=5
Done


---

What This Is (and Is Not)

This is

A reusable native engine core

A proven Rust ↔ C ↔ .NET bridge

A foundation for protocol engines, simulators, game logic, or SDK cores

Explicitly designed for desktop + mobile parity


This is not

A UI framework

A build automation showcase

A platform-specific app

A language binding generator



---

Design Goals

One core, many hosts

Stable ABI over time

No shared memory assumptions

Explicit ownership

Predictable failure modes



---

Future Directions (Not Yet Implemented)

Structured error reporting

Binary message payloads

Event/callback pumps

Deterministic replay

WASM target

Additional host bindings


These can be added without breaking the ABI.


---

License

TBD (intentionally omitted until API stabilizes).


---
