use std::ffi::c_int;
use std::ptr::NonNull;

#[repr(C)]
pub struct BridgeContext {
    value: c_int,
}

#[unsafe(export_name = "bridge_create")]
pub extern "C" fn bridge_create() -> *mut BridgeContext {
    let ctx = Box::new(BridgeContext { value: 0 });
    Box::into_raw(ctx)
}

#[unsafe(export_name = "bridge_destroy")]
pub extern "C" fn bridge_destroy(ctx: *mut BridgeContext) {
    if let Some(ctx) = NonNull::new(ctx) {
        unsafe {
            drop(Box::from_raw(ctx.as_ptr()));
        }
    }
}

#[unsafe(export_name = "bridge_add")]
pub extern "C" fn bridge_add(ctx: *mut BridgeContext, x: c_int) -> c_int {
    let ctx = unsafe { &mut *ctx };
    ctx.value += x;
    ctx.value
}
