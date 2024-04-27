#![allow(unused)]
#![allow(nonstandard_style)]

extern crate alloc;

use core::{
    ffi::{c_int, c_void},
    fmt::Display,
    iter,
    mem::{size_of, size_of_val},
};

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::{
    alloc::{GlobalAlloc, Layout},
    vec::Vec,
};

use windows_sys::Win32::{
    Foundation::GlobalFree,
    Globalization::{lstrcpyW, lstrcpynW},
    System::Memory::{
        GetProcessHeap, GlobalAlloc, HeapAlloc, HeapFree, HeapReAlloc, GPTR, HEAP_ZERO_MEMORY,
    },
};

pub type wchar_t = i32;

#[repr(C)]
#[derive(Debug)]
pub struct stack_t {
    pub next: *mut stack_t,
    pub text: [wchar_t; 1],
}

pub static mut G_STRINGSIZE: c_int = 0;
pub static mut G_VARIABLES: *mut wchar_t = core::ptr::null_mut();
pub static mut G_STACKTOP: *mut *mut stack_t = core::ptr::null_mut();

#[inline(always)]
pub unsafe fn exdll_init(string_size: c_int, variables: *mut wchar_t, stacktop: *mut *mut stack_t) {
    G_STRINGSIZE = string_size;
    G_VARIABLES = variables;
    G_STACKTOP = stacktop;
}

pub const ONE: [u16; 2] = [49, 0];
pub const ZERO: [u16; 2] = [48, 0];
pub const NEGATIVE_ONE: [u16; 3] = [45, 49, 0];

#[derive(Debug)]
pub enum Error {
    StackIsNull,
    ParseIntError,
}

impl Error {
    const fn description(&self) -> &str {
        match self {
            Error::StackIsNull => "Stack is null",
            Error::ParseIntError => "Failed to parse integer",
        }
    }
    pub fn push_err(&self) {
        let _ = unsafe { pushstr(&self.description()) };
    }
}

pub unsafe fn pushptr(ptr: *const u16) -> Result<(), Error> {
    if G_STACKTOP.is_null() {
        return Err(Error::StackIsNull);
    }

    let n = size_of::<stack_t>() + G_STRINGSIZE as usize * 2;
    let th = GlobalAlloc(GPTR, n) as *mut stack_t;
    lstrcpyW((*th).text.as_ptr() as _, ptr);
    (*th).next = *G_STACKTOP;
    *G_STACKTOP = th;

    Ok(())
}

pub unsafe fn push(bytes: &[u16]) -> Result<(), Error> {
    pushptr(bytes.as_ptr())
}

pub unsafe fn pushstr(str: &str) -> Result<(), Error> {
    let bytes = encode_wide(str);
    push(&bytes)
}

pub unsafe fn pushint(int: i32) -> Result<(), Error> {
    let str = int.to_string();
    pushstr(&str)
}

pub unsafe fn pop() -> Result<Vec<u16>, Error> {
    if G_STACKTOP.is_null() || (*G_STACKTOP).is_null() {
        return Err(Error::StackIsNull);
    }

    let mut out = vec![0_u16; G_STRINGSIZE as _];

    let th: *mut stack_t = *G_STACKTOP;
    lstrcpyW(out.as_mut_ptr(), (*th).text.as_ptr() as _);
    *G_STACKTOP = (*th).next;
    GlobalFree(th as _);

    Ok(out)
}

pub unsafe fn popstr() -> Result<String, Error> {
    let bytes = pop()?;
    Ok(decode_wide(&bytes))
}

pub unsafe fn popint() -> Result<i32, Error> {
    let str = popstr()?;
    str.parse().map_err(|_| Error::ParseIntError)
}

pub fn encode_wide(str: &str) -> Vec<u16> {
    str.encode_utf16()
        .chain(iter::once(0))
        .collect::<Vec<u16>>()
}

pub fn decode_wide(bytes: &[u16]) -> String {
    let bytes = bytes
        .iter()
        .position(|c| *c == 0)
        .map(|nul| &bytes[..nul])
        .unwrap_or(&bytes);
    String::from_utf16_lossy(bytes)
}
