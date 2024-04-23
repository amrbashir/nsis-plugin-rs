#![allow(unused)]
#![allow(nonstandard_style)]

use core::{
    ffi::c_int,
    fmt::Display,
    mem::{size_of, size_of_val},
};

use alloc::string::{String, ToString};
use alloc::vec;
use widestring::U16CString;
use windows_sys::Win32::{
    Foundation::GlobalFree,
    Globalization::{lstrcpyW, lstrcpynW},
    System::Memory::{GlobalAlloc, GPTR},
};

pub type wchar_t = i32;

#[derive(Debug)]
pub enum Error {
    StackIsNull,
    ParseIntError(core::num::ParseIntError),
    StrContainsNul(widestring::error::ContainsNul<u16>),
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::StackIsNull => write!(f, "Stack is null"),
            Error::ParseIntError(e) => write!(f, "{}", e.to_string()),
            Error::StrContainsNul(e) => write!(f, "{}", e.to_string()),
        }
    }
}

impl From<core::num::ParseIntError> for Error {
    fn from(value: core::num::ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl From<widestring::error::ContainsNul<u16>> for Error {
    fn from(value: widestring::error::ContainsNul<u16>) -> Self {
        Self::StrContainsNul(value)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct stack_t {
    pub next: *mut stack_t,
    pub text: [wchar_t; 1],
}

static mut G_STRINGSIZE: c_int = 0;
static mut G_VARIABLES: *mut wchar_t = core::ptr::null_mut();
static mut G_STACKTOP: *mut *mut stack_t = core::ptr::null_mut();

pub unsafe fn exdll_init(string_size: c_int, variables: *mut wchar_t, stacktop: *mut *mut stack_t) {
    G_STRINGSIZE = string_size;
    G_VARIABLES = variables;
    G_STACKTOP = stacktop;
}
pub unsafe fn pushstring(string: &str) -> Result<(), Error> {
    if G_STACKTOP.is_null() {
        return Err(Error::StackIsNull);
    }

    let string = U16CString::from_str(string)?;

    let n = size_of::<stack_t>() + G_STRINGSIZE as usize * size_of_val(&string);
    let th = GlobalAlloc(GPTR, n) as *mut stack_t;
    lstrcpynW((*th).text.as_ptr() as _, string.as_ptr(), G_STRINGSIZE as _);

    (*th).next = *G_STACKTOP;
    *G_STACKTOP = th;

    Ok(())
}

pub unsafe fn popstring() -> Result<String, Error> {
    if G_STACKTOP.is_null() || (*G_STACKTOP).is_null() {
        return Err(Error::StackIsNull);
    }

    let th: *mut stack_t = *G_STACKTOP;

    let mut out = vec![0_u16; G_STRINGSIZE as _];
    lstrcpyW(out.as_mut_ptr(), (*th).text.as_ptr() as _);

    *G_STACKTOP = (*th).next;

    GlobalFree(th as _);

    decode_wide(&out)
}

pub unsafe fn popint() -> Result<i32, Error> {
    popstring().and_then(|i| i.parse().map_err(Into::into))
}

pub unsafe fn pushint(int: i32) -> Result<(), Error> {
    pushstring(&int.to_string())
}

pub fn decode_wide(mut wide_c_string: &[u16]) -> Result<String, Error> {
    if let Some(null_pos) = wide_c_string.iter().position(|c| *c == 0) {
        wide_c_string = &wide_c_string[..null_pos];
    }

    U16CString::from_vec(wide_c_string)
        .map(|s| s.to_string_lossy())
        .map_err(Into::into)
}
