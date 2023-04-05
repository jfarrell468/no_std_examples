# no_std_examples

Four examples of `#![no_std]` Rust programs, progressively building to a point where we have a pretty normal looking `main` function, with command line args, `Vec`, `String`, etc.

Requires x86_64 Linux.

To run:

```
cargo run --target x86_64-unknown-none --bin 01-hello-world
cargo run --target x86_64-unknown-none --bin 02-hexdump-stack foo bar
cargo run --target x86_64-unknown-none --bin 03-args foo bar
cargo run --target x86_64-unknown-none --bin 04-allocator foo bar
```

## 1. Hello, world

[Source code](src/bin/01-hello-world.rs)

This is basically taken from [Writing a Linux executable from scratch with x86_64-unknown-none and Rust](https://vulns.xyz/2023/03/linux-executable-from-scratch-with-x86_64-unknown-none-rust/).

Program execution begins with the `_start` symbol, which we define in assembly and use to call the `start_main` function, passing the address of the top of the stack. Now we are in Rust land.

To output "Hello, world", we use the [write](https://man7.org/linux/man-pages/man2/write.2.html) syscall, and we use the [exit](https://man7.org/linux/man-pages/man2/exit.2.html) syscall to terminate the program. Both of these are implemented in assembly.

Output:

```
$ cargo -q run --target x86_64-unknown-none --bin 01-hello-world
Hello, world
```

## 2. Examining the stack

[Source code](src/bin/02-hexdump-stack)

Now we want to take a look at the stack and see what it contains. This is accomplished with a hexdump function. While we could implement all of this manually, it's nice to be able to use formatted output to render things in hexadecimal. For this we use the [ufmt](https://docs.rs/ufmt/latest/ufmt/) crate.

We also need a place to store the formatted output so we can print it. Normally, we would use a `String` or perhaps a `Vec<u8>`, but those are both "growable", without a predetermined size, so they require a heap and an allocator, neither of which we have. (For the same reason, we can't use the standard [alloc::fmt](https://doc.rust-lang.org/beta/alloc/fmt/index.html) for formatted output.) So, instead, we use a fixed-size array.

When we run it with `cargo -q run --target x86_64-unknown-none --bin 02-hexdump-stack foo bar`, the first three lines of output are:

```
00007ffe308c7290: 0300000000000000 000000001b958c30 1b958c30fe7f0000 fe7f00004d958c30  ...........0....M..0....Q..0....
00007ffe308c72b0: 0000000000000000 0000000055958c30 55958c30fe7f0000 fe7f00008e958c30  ........U..0.......0.......0....
00007ffe308c72d0: 2b968c30fe7f0000 fe7f000076968c30 76968c30fe7f0000 fe7f000089968c30  +..0....v..0.......0.......0....
```

Here, we can see that the stack contains:

1. In word 1, the number of command-line arguments (3), which becomes `argc` in C.
2. In the next 3 words, the addresses of the values of the command-line arguments (i.e. pointers to null-terminated C-style strings). In C, we call this `argv`.
3. A word with value 0, indicating the end of the argument value array.

(After this, we have a second array of memory addresses which point to the addresses of the environment variables. We're going to ignore that for this example.)

## 3. Parsing the command-line arguments

[Source code](src/bin/03-args)

Next, let's turn the command-line into something easier to work with: A slice of pointers to bytes, or `[*const u8]`. The length of the slice is the number of command-line arguments, and each element is a pointer to a null-terminated string. We can turn those into slices as well by finding the next null value, which gives us the length.

Now we can print the command-line arguments.

```
$ cargo -q run --target x86_64-unknown-none --bin 03-args foo bar
argc = 3
argv[0] = target/x86_64-unknown-none/debug/03-args
argv[1] = foo
argv[2] = bar
```

## 4. Adding an allocator

[Source code](src/bin/04-allocator)

Instead of `[*const u8]`, it would be much nicer if the command-line arguments were something more Rust-like: `Vec<&str>`. To use `Vec`, we need an allocator, for which we can use the [wee_alloc](https://docs.rs/wee_alloc/0.4.5/wee_alloc/) crate, which is a simple allocator that can be backed by fixed-size array. This also let us use most "normal" Rust types like `String`, collection types, etc.

In addition, we make several ergonomic improvements:

* Move the process startup code into a separate file, [begin.rs](src/bin/04-allocator/begin.rs) and have `start_main` call a more normal-looking `main` function.
* Add `uprint!` and `uprintln!` macros, similar to the standard `print!` and `println!`, by using a `String` to accumulate the formatted output.
* Make the command line accessible with `env::args()`, similar to how we can in [regular Rust](https://doc.rust-lang.org/stable/std/env/fn.args.html). Our code is a simplified version of how Rust handles [command lines on Unix](https://github.com/rust-lang/rust/blob/master/library/std/src/sys/unix/args.rs).

Now, we have a main function to print the command line arguments that looks pretty darn Rust-like:

```rust
fn main() {
    let argv = env::args();

    uprintln!("argc = {}", argv.len());

    for (i, arg) in argv.iter().enumerate() {
        uprintln!("argv[{}] = {}", i, arg);
    }
}
```

Amazing!
