use core::cmp;
use core::fmt::{Error, Write};

use crate::syscalls::write;

pub struct Cursor<'a> {
    buf: &'a mut [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Cursor {
            buf: buf,
            offset: 0,
        }
    }
    pub fn reset(&mut self) {
        self.offset = 0;
    }
    pub fn print(&self, fd: i32) -> isize {
        write(fd, &self.buf[..self.offset])
    }
}

impl<'a> Write for Cursor<'a> {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        let bytes = s.as_bytes();

        // Skip over already-copied data
        let remainder = &mut self.buf[self.offset..];
        // Only copy what there is room for.
        let bytes_to_copy = cmp::min(remainder.len(), bytes.len());

        // Make the two slices the same length
        let bytes = &bytes[..bytes_to_copy];
        let remainder = &mut remainder[..bytes_to_copy];

        // Copy
        remainder.copy_from_slice(bytes);

        // Update offset to avoid overwriting
        self.offset += bytes_to_copy;

        Ok(())
    }
}
