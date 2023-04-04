extern crate alloc;
use alloc::vec::Vec;
use core::ptr;
use core::slice::from_raw_parts;
use core::str::from_utf8;
use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

static ARGC: AtomicUsize = AtomicUsize::new(0);
static ARGV: AtomicPtr<*const u8> = AtomicPtr::new(ptr::null_mut());

pub unsafe fn init(stack_top: *const u8) {
    ARGC.store(*(stack_top as *const usize), Ordering::Relaxed);
    ARGV.store((stack_top as *mut *const u8).offset(1), Ordering::Relaxed);
}

unsafe fn byte_slice_from_null_terminated<'a>(src: *const u8) -> &'a [u8] {
    const MAX_LEN: usize = 1024;
    for i in 0..MAX_LEN {
        if *src.offset(i as isize) == 0 {
            return from_raw_parts(src, i);
        }
    }
    from_raw_parts(src, MAX_LEN)
}

pub fn args() -> Vec<&'static str> {
    let argc = ARGC.load(Ordering::Relaxed);
    let argv = unsafe { from_raw_parts(ARGV.load(Ordering::Relaxed) as *const *const u8, argc) };
    let mut v = Vec::with_capacity(argc);
    for i in 0..argc {
        unsafe { v.push(from_utf8(byte_slice_from_null_terminated(argv[i])).unwrap_or_default()) }
    }
    v
}
