#![no_std]
#![no_main]

mod cursor;
mod syscalls;

use core::arch::global_asm;
use core::panic::PanicInfo;
use core::slice::from_raw_parts;
use core::str::from_utf8;
use ufmt::uwriteln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    syscalls::exit(1);
}

global_asm! {
    ".global _start",
    "_start:",
    "mov rdi, rsp",
    "call start_main"
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

#[no_mangle]
unsafe fn start_main(stack_top: *const u8) -> ! {
    let argv = from_raw_parts(
        (stack_top as *const *const u8).offset(1),
        *(stack_top as *const usize),
    );

    let mut buf: [u8; 128] = [0; 128];
    let mut cursor = cursor::Cursor::new(&mut buf);

    uwriteln!(cursor, "argc = {}", argv.len()).unwrap_unchecked();
    cursor.print(1);

    for i in 0..argv.len() {
        cursor.reset();
        uwriteln!(
            cursor,
            "argv[{}] = {}",
            i,
            from_utf8(byte_slice_from_null_terminated(argv[i])).unwrap()
        )
        .unwrap_unchecked();
        cursor.print(1);
    }

    syscalls::exit(0);
}
