use std::ffi::c_int;

#[unsafe(no_mangle)]
pub extern "C" fn bridge_add(a: c_int, b: c_int) -> c_int {
    a + b
}
