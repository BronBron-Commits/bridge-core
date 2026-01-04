Usage Guide

This document describes how hosts are expected to drive bridge-core.

Typical Host Flow

1. Load the native library
2. Verify the ABI version
3. Create a context
4. Issue commands
5. Destroy the context

Example Flow

Create context
Verify ABI equals 1

Send Add command with value 2
Receive result 2

Send Add command with value 3
Receive result 5

Send Step command
Receive tick value

Destroy context

Threading Rules

- A BridgeContext is not thread-safe
- A context must only be accessed by one thread at a time
- Multiple contexts may exist concurrently

Lifetime Rules

- Never reuse a destroyed context
- Never share a context across language runtimes
- Never assume pointer stability after destroy
