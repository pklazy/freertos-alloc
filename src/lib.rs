#![no_std]

use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;

pub struct Freertosllocator;

extern "C" {
    fn pvPortMalloc(size: usize) -> *mut c_void;
    fn vPortFree(ptr: *mut c_void);
}

unsafe impl GlobalAlloc for Freertosllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        pvPortMalloc(layout.size()) as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        vPortFree(ptr as *mut c_void)
    }
}
