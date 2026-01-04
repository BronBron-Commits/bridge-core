C ABI Contract

This document defines the stable ABI exposed by bridge-core.

Anything listed here is considered part of the compatibility contract.

Versioning

Function: bridge_abi_version
Returns the ABI version as an unsigned 32-bit integer.

Current ABI version: 1

Hosts must verify this before issuing commands.

Context Lifecycle

Functions:
bridge_create
bridge_destroy

Rules:
- bridge_create returns ownership to the caller
- bridge_destroy must be called exactly once
- Passing a null context is allowed and ignored

Command Dispatcher

Function:
bridge_command

Parameters:
- context pointer
- command identifier
- signed 64-bit argument
- pointer to signed 64-bit output value

Return Values
0  success
negative values indicate error

Error Codes
-10  null context
-20  unknown command
-99  panic caught

Commands

ID 1  Step     advances the internal tick counter
ID 2  Reset    resets all internal state
ID 3  GetLast  returns the last computed value
ID 4  Add      adds the argument to internal state

ABI Stability Rules

- No struct layout changes
- No removal of commands
- New commands must use new IDs
- Existing command semantics must not change
