extern crate alloc;
use alloc::string::String;
use core::convert::Infallible;
use ufmt::uWrite;

pub struct StringWriter(pub String);

impl uWrite for StringWriter {
    type Error = Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Infallible> {
        self.0.push_str(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! uprint {
    ($($arg:tt)*) => {{
        extern crate alloc;
        use alloc::string::String;
        use ufmt::uwrite;
        use crate::syscalls::write;
        use crate::print::StringWriter;
        let mut s = StringWriter(String::new());
        uwrite!(&mut s, $($arg)*).unwrap();
        write(1, s.0.as_bytes());
    }};
}

#[macro_export]
macro_rules! uprintln {
    ($($arg:tt)*) => {{
        extern crate alloc;
        use alloc::string::String;
        use ufmt::uwriteln;
        use crate::syscalls::write;
        use crate::print::StringWriter;
        let mut s = StringWriter(String::new());
        uwriteln!(&mut s, $($arg)*).unwrap();
        write(1, s.0.as_bytes());
    }};
}
