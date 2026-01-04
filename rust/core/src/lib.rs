use std::ffi::c_int;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr::NonNull;

#[repr(C)]
pub struct BridgeContext {
    tick: u64,
    last_result: i64,
}

#[repr(u32)]
pub enum BridgeCommand {
    Step = 1,
    Reset = 2,
    GetLast = 3,
    Add = 4, // arg = value to add
}

#[unsafe(export_name = "bridge_abi_version")]
pub extern "C" fn bridge_abi_version() -> u32 {
    1
}

#[unsafe(export_name = "bridge_create")]
pub extern "C" fn bridge_create() -> *mut BridgeContext {
    let ctx = Box::new(BridgeContext {
        tick: 0,
        last_result: 0,
    });
    Box::into_raw(ctx)
}

#[unsafe(export_name = "bridge_destroy")]
pub extern "C" fn bridge_destroy(ctx: *mut BridgeContext) {
    if let Some(ctx) = NonNull::new(ctx) {
        unsafe { drop(Box::from_raw(ctx.as_ptr())); }
    }
}

/// Unified command entry point
/// Returns 0 on success, negative on error
#[unsafe(export_name = "bridge_command")]
pub extern "C" fn bridge_command(
    ctx: *mut BridgeContext,
    cmd: u32,
    arg: i64,
    out: *mut i64,
) -> c_int {
    if ctx.is_null() {
        return -10; // null ctx
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        let ctx = unsafe { &mut *ctx };

        match cmd {
            x if x == BridgeCommand::Step as u32 => {
                ctx.tick += 1;
                if !out.is_null() {
                    unsafe { *out = ctx.tick as i64; }
                }
                0
            }
            x if x == BridgeCommand::Reset as u32 => {
                ctx.tick = 0;
                ctx.last_result = 0;
                if !out.is_null() {
                    unsafe { *out = 0; }
                }
                0
            }
            x if x == BridgeCommand::GetLast as u32 => {
                if !out.is_null() {
                    unsafe { *out = ctx.last_result; }
                }
                0
            }
            x if x == BridgeCommand::Add as u32 => {
                ctx.last_result += arg;
                if !out.is_null() {
                    unsafe { *out = ctx.last_result; }
                }
                0
            }
            _ => -20, // unknown command
        }
    }));

    match result {
        Ok(code) => code,
        Err(_) => -99, // panic caught
    }
}
