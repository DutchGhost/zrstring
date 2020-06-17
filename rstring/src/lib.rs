#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc_geiger::Geiger;
use jemallocator::Jemalloc;

#[global_allocator]
static ALLOC: Geiger<Jemalloc> = Geiger::new(Jemalloc);

use core::{
    mem::{ManuallyDrop, MaybeUninit},
    ptr, slice, str,
};

#[no_mangle]
pub unsafe extern "C" fn string_new(this: &mut MaybeUninit<String>) {
    ptr::write(this.as_mut_ptr(), String::new());
}

#[no_mangle]
pub extern "C" fn string_ptr(this: &String) -> *const u8 {
    this.as_ptr()
}

#[no_mangle]
pub extern "C" fn string_len(this: &String) -> usize {
    this.len()
}

#[no_mangle]
pub extern "C" fn string_push(this: &mut ManuallyDrop<String>, ch: char) {
    this.push(ch)
}

#[no_mangle]
pub unsafe extern "C" fn string_drop(this: &mut ManuallyDrop<String>) {
    ManuallyDrop::drop(this)
}

#[no_mangle]
pub unsafe extern "C" fn string_push_str(
    this: &mut ManuallyDrop<String>,
    ptr: *const u8,
    len: usize,
) {
    let slice = slice::from_raw_parts(ptr, len);
    let s = str::from_utf8_unchecked(slice);

    this.push_str(s);
}
