use core::arch::global_asm;
use core::panic::PanicInfo;

use crate::env;
use crate::syscalls::exit;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    crate::syscalls::exit(1);
}

global_asm! {
    ".global _start",
    "_start:",
    "mov rdi, rsp",
    "call start_main"
}

extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
unsafe fn start_main(stack_top: *const u8) -> ! {
    env::init(stack_top);
    crate::main();
    exit(0);
}
