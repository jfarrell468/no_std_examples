#![no_std]
#![no_main]

mod cursor;
mod hexdump;
mod syscalls;

use core::arch::global_asm;
use core::panic::PanicInfo;
use core::slice::from_raw_parts;

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

#[no_mangle]
unsafe fn start_main(stack_top: *const u8) -> ! {
    hexdump::dump_memory(from_raw_parts(stack_top, 1024));
    syscalls::exit(0);
}
