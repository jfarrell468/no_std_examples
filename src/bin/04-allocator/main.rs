#![no_std]
#![no_main]

mod begin;
mod env;
mod print;
mod syscalls;

fn main() {
    let argv = env::args();

    println!("argc = {}", argv.len());

    for (i, arg) in argv.iter().enumerate() {
        println!("argv[{}] = {}", i, arg);
    }
}
