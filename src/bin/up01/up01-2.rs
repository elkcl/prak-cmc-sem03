use std::{io, slice};
use std::io::Read;

const XOR_MASK: u8 = 1u8 << 3;
const AND_MASK: u8 = !(1u8 << 2);
const CODE_AT: u8 = 0;
const CODE_DIGITS: u8 = 1;
const CODE_LOWERCASE: u8 = 11;
const CODE_UPPERCASE: u8 = 37;
const CODE_SHARP: u8 = 63;

fn main() {
    let stdin = io::stdin();
    let mut c: u8 = b'\0';
    while stdin
        .lock()
        .read(slice::from_mut(&mut c))
        .expect("Input error")
        > 0 {
        let mut res: u8 = c;
        if b'0' <= c && c <= b'9' {

        }
    }
}