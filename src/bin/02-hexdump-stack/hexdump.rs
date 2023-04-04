use core::ptr::addr_of;
use ufmt::{uWrite, uwrite};

use crate::cursor::Cursor;

pub fn dump_memory(buf: &[u8]) {
    const BYTES_PER_WORD: usize = 8;
    const WORDS_PER_LINE: usize = 4;
    const BYTES_PER_LINE: usize = BYTES_PER_WORD * WORDS_PER_LINE;
    const OUTPUT_BYTES_PER_LINE: usize = 2 * BYTES_PER_WORD                         // address
                                         + 2                                        // ": "
                                         + 2 * BYTES_PER_LINE + WORDS_PER_LINE - 1  // hex, words separated by spaces
                                         + 2                                        // two spaces
                                         + BYTES_PER_LINE                           // printable bytes
                                         + 1; // newline

    let lines = (buf.len() + BYTES_PER_LINE - 1) / BYTES_PER_LINE;
    for i in 0..lines {
        let mut line: [u8; OUTPUT_BYTES_PER_LINE] = [0; OUTPUT_BYTES_PER_LINE];
        let mut cursor = Cursor::new(&mut line);
        uwrite!(
            cursor,
            "{:016x}: ",
            addr_of!(buf[i * BYTES_PER_LINE]) as usize
        )
        .unwrap();
        for j in 0..WORDS_PER_LINE {
            for k in 0..BYTES_PER_WORD {
                let offset = i * BYTES_PER_LINE + j * WORDS_PER_LINE + k;
                if offset < buf.len() {
                    uwrite!(cursor, "{:02x}", buf[offset]).unwrap();
                } else {
                    uwrite!(cursor, "  ").unwrap();
                }
            }
            uwrite!(cursor, " ").unwrap();
        }
        uwrite!(cursor, " ").unwrap();
        for j in 0..BYTES_PER_LINE {
            let offset = i * BYTES_PER_LINE + j;
            if offset < buf.len() {
                let c = buf[offset];
                cursor
                    .write_char(if is_printable(c) { c as char } else { '.' })
                    .unwrap();
            } else {
                cursor.write_char(' ').unwrap();
            }
        }
        uwrite!(cursor, "\n").unwrap();
        cursor.print(1);
    }
}

fn is_printable(b: u8) -> bool {
    b >= 32 && b <= 126
}
