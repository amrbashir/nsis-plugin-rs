#![no_std]

extern crate alloc;

mod nsis_plugin_api;
mod win32alloc;

use core::ffi::c_int;

use windows_sys::Win32::{Foundation::HWND, Globalization::lstrcatW};

use nsis_plugin_api::*;

/// Greet a person
///
/// # Safety
///
/// Top of the stack should be a valid string
#[no_mangle]
pub unsafe extern "C" fn greet(
    _hwnd_parent: HWND,
    string_size: c_int,
    variables: *mut wchar_t,
    stacktop: *mut *mut stack_t,
) {
    exdll_init(string_size, variables, stacktop);

    let name = match pop() {
        Ok(s) => s,
        Err(e) => {
            e.push_err();
            return;
        }
    };

    const HELLO: &str = "Hello ";
    let mut hello = encode_wide(HELLO);

    let concatenanted = lstrcatW(hello.as_mut_ptr(), name.as_ptr());

    if let Err(e) = pushptr(concatenanted) {
        e.push_err();
    }
}

#[no_mangle]
extern "system" fn DllMain(
    _dll_module: ::windows_sys::Win32::Foundation::HINSTANCE,
    _call_reason: u32,
    _: *mut (),
) -> bool {
    true
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { ::windows_sys::Win32::System::Threading::ExitProcess(u32::MAX) }
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    return dest;
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);
        if a != b {
            return a as i32 - b as i32;
        }
        i += 1;
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }
    return s;
}

// #[no_mangle]
// pub unsafe extern fn memmove(dest: *mut u8, src: *const u8,
//                              n: usize) -> *mut u8 {
//     if src < dest as *const u8 { // copy from end
//         let mut i = n;
//         while i != 0 {
//             i -= 1;
//             *dest.offset(i as isize) = *src.offset(i as isize);
//         }
//     } else { // copy from beginning
//         let mut i = 0;
//         while i < n {
//             *dest.offset(i as isize) = *src.offset(i as isize);
//             i += 1;
//         }
//     }
//     return dest;
// }
