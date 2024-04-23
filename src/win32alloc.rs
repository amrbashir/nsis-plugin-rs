use core::ffi::c_void;

use alloc::alloc::{GlobalAlloc, Layout};
use windows_sys::Win32::System::Memory::{
    GetProcessHeap, HeapAlloc, HeapFree, HeapReAlloc, HEAP_ZERO_MEMORY,
};

#[global_allocator]
static WIN32_STD_ALLOCATOR: Heapalloc = Heapalloc;

pub struct Heapalloc;

unsafe impl GlobalAlloc for Heapalloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        HeapAlloc(GetProcessHeap(), 0, _layout.size()) as *mut u8
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        HeapAlloc(GetProcessHeap(), HEAP_ZERO_MEMORY, layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        HeapFree(GetProcessHeap(), 0, _ptr as *mut c_void);
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        HeapReAlloc(
            GetProcessHeap(),
            HEAP_ZERO_MEMORY,
            ptr as *mut c_void,
            new_size,
        ) as *mut u8
    }
}
