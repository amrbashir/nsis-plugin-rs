mod utils;

use utils::{exdll_init, popstring, pushstring, stack_t, wchar_t};
use windows_sys::Win32::Foundation::HWND;

#[no_mangle]
pub unsafe extern "C" fn greet(
    _hwnd_parent: HWND,
    string_size: u32,
    variables: *mut wchar_t,
    stacktop: *mut *mut stack_t,
) {
    exdll_init(string_size, variables, stacktop);

    let name = popstring().unwrap();
    pushstring(format!("Hello {}!", name));
}
