use core::arch::asm;

pub fn exit(status: i32) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") 60,
            in("rdi") status,
            options(noreturn)
        );
    }
}

pub fn write(fd: i32, buf: &[u8]) -> isize {
    let r0;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") 1isize => r0,
            in("rdi") fd,
            in("rsi") buf.as_ptr(),
            in("rdx") buf.len(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
    }
    r0
}
