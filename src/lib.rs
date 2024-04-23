#![no_std]
#![no_main]

extern crate alloc;

mod utils;
mod win32alloc;

use core::ffi::c_int;

use alloc::format;
use alloc::string::ToString;
use utils::*;
use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::System::Threading::ExitProcess;

/// Greet a person
///
/// # Safety
///
/// Top of the stack should be a valid string
#[no_mangle]
pub unsafe extern "C" fn greet(
    _hwnd_parent: HWND,
    string_size: c_int,
    variables: *mut i32,
    stacktop: *mut *mut stack_t,
) {
    exdll_init(string_size, variables, stacktop);

    let name = match popstring() {
        Ok(s) => s,
        Err(e) => e.to_string(),
    };
    let out = format!("Hello {name}");
    pushstring(&out).unwrap()
}

#[no_mangle]
extern "C" fn main() -> i32 {
    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { ExitProcess(u32::MAX) }
}

// wrong signature but shouldn't matter
#[no_mangle]
extern "C" fn __CxxFrameHandler3() {
    unsafe { ExitProcess(u32::MAX) };
}
