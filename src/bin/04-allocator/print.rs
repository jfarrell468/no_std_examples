extern crate alloc;
use alloc::string::String;

pub struct StringWriter(pub String);

impl core::fmt::Write for StringWriter {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        self.0.push_str(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        extern crate alloc;
        use alloc::string::String;
        use crate::print::StringWriter;
        let mut s = StringWriter(String::new());
        core::fmt::write(&mut s, format_args!($($arg)*)).unwrap();
        crate::syscalls::write(1, s.0.as_bytes());
    }};
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        extern crate alloc;
        use alloc::string::String;
        use crate::print::StringWriter;
        let mut s = StringWriter(String::new());
        core::fmt::write(&mut s, format_args!($($arg)*)).unwrap();
        s.0.push_str("\n");
        crate::syscalls::write(1, s.0.as_bytes());
    }};
}
