use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
    ptr,
};

use windows_sys::Win32::{
    Foundation::INVALID_HANDLE_VALUE,
    System::{
        Console::GetStdHandle,
        Memory::{GetProcessHeap, HeapAlloc, HeapFree},
    },
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlen(ptr: *const u8) -> usize {
    let mut len = 0;
    // FIX: We must wrap the pointer arithmetic in unsafe block,
    // even though the function itself is marked unsafe.
    unsafe {
        while *ptr.add(len) != 0 {
            len += 1;
        }
    }
    len
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn WriteFile(
        hFile: *mut c_void,
        lpBuffer: *const u8,
        nNumberOfBytesToWrite: u32,
        lpNumberOfBytesWritten: *mut u32,
        lpOverlapped: *mut c_void,
    ) -> i32;
}

#[inline(always)]
pub fn print_to_handle(handle_type: u32, s: &str) {
    unsafe {
        let handle = GetStdHandle(handle_type);
        if handle != core::ptr::null_mut() && handle != INVALID_HANDLE_VALUE as _ {
            let mut written = 0;
            WriteFile(
                handle,
                s.as_ptr(),
                s.len() as u32,
                &mut written,
                ptr::null_mut(),
            );
        }
    }
}

#[inline(always)]
pub unsafe fn zero_memory(ptr: *mut u8, len: usize) {
    unsafe {
        for i in 0..len {
            core::ptr::write_volatile(ptr.add(i), 0);
        }
    }
}

pub struct Win32HeapAllocator;

unsafe impl GlobalAlloc for Win32HeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { HeapAlloc(GetProcessHeap(), 0, layout.size()) as *mut u8 }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe {
            HeapFree(GetProcessHeap(), 0, ptr as *mut c_void);
        }
    }
}
