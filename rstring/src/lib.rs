#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc_geiger::Geiger;
use jemallocator::Jemalloc;

#[global_allocator]
static ALLOC: Geiger<Jemalloc> = Geiger::new(Jemalloc);

use core::{
    mem::{self, ManuallyDrop},
    slice, str,
};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FFIString {
    ptr: *mut u8,
    len: usize,
    cap: usize,
}

impl FFIString {
    #[inline(always)]
    pub fn new() -> Self {
        String::new().into()
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *const u8 {
        self.access(|s| s.as_ptr())
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.access(|s| s.len())
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.access(|s| s.is_empty())
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.access(|s| s.capacity())
    }

    #[inline(always)]
    pub fn push(&mut self, ch: char) {
        self.access_mut(|s| s.push(ch));
    }

    #[inline(always)]
    pub fn push_str(&mut self, slice: &str) {
        self.access_mut(|s| s.push_str(slice))
    }

    #[inline(always)]
    pub fn dropping(&mut self) {
        self.access_mut(|s| mem::take(s));
    }
}

impl Default for FFIString {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl FFIString {
    #[inline(always)]
    fn access<O>(&self, f: impl FnOnce(&String) -> O) -> O {
        let s = ManuallyDrop::new(unsafe { self.into_string_unchecked() });
        f(&s)
    }

    #[inline(always)]
    fn access_mut<O>(&mut self, f: impl FnOnce(&mut String) -> O) -> O {
        let mut s = ManuallyDrop::new(unsafe { self.into_string_unchecked() });
        let out = f(&mut s);
        *self = ManuallyDrop::into_inner(s).into();
        out
    }

    #[inline(always)]
    unsafe fn into_string_unchecked(self) -> String {
        String::from_raw_parts(self.ptr, self.len, self.cap)
    }
}

impl From<String> for FFIString {
    #[inline(always)]
    fn from(s: String) -> Self {
        let mut s = ManuallyDrop::new(s);
        let (ptr, len, cap) = (s.as_mut_ptr(), s.len(), s.capacity());
        Self { ptr, len, cap }
    }
}

#[no_mangle]
pub extern "C" fn string_new() -> FFIString {
    FFIString::new()
}

#[no_mangle]
pub extern "C" fn string_ptr(this: &FFIString) -> *const u8 {
    FFIString::as_ptr(this)
}

#[no_mangle]
pub extern "C" fn string_len(this: &FFIString) -> usize {
    FFIString::len(this)
}

#[no_mangle]
pub extern "C" fn string_push(this: &mut FFIString, ch: char) {
    FFIString::push(this, ch)
}

#[no_mangle]
pub extern "C" fn string_drop(this: &mut FFIString) {
    FFIString::dropping(this)
}

/// # Safety
/// The pointer `ptr` must be a valid pointer to `len` bytes.
#[no_mangle]
pub unsafe extern "C" fn string_push_str(this: &mut FFIString, ptr: *const u8, len: usize) {
    let strslice = {
        let slice = slice::from_raw_parts(ptr, len);
        str::from_utf8_unchecked(slice)
    };

    FFIString::push_str(this, strslice)
}
