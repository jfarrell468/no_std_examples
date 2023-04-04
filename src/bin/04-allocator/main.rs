#![no_std]
#![no_main]

mod begin;
mod env;
mod print;
mod syscalls;

fn main() {
    let argv = env::args();

    uprintln!("argc = {}", argv.len());

    for (i, arg) in argv.iter().enumerate() {
        uprintln!("argv[{}] = {}", i, arg);
    }
}
