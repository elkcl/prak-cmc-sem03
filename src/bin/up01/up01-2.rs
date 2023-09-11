use std::io;
use std::io::{Read, Write};

const XOR_MASK: i32 = 1 << 3;
const AND_MASK: i32 = !(1 << 2);
const CODE_AT: i32 = 0;
const CODE_DIGITS: i32 = 1;
const CODE_LOWERCASE: i32 = 11;
const CODE_UPPERCASE: i32 = 37;
const CODE_SHARP: i32 = 63;

fn getchar(h: &mut impl Read) -> Option<u8> {
    let mut buf: [u8; 1] = [b'\0'];
    if h.read(&mut buf).expect("Input error") > 0 {
        Some(buf[0])
    } else {
        None
    }
}

fn putchar(h: &mut impl Write, c: u8) -> () {
    let buf: [u8; 1] = [c];
    h.write(&buf).expect("Output error");
}

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    while let Some(c) = getchar(&mut stdin) {
        let mut res: i32 = c as i32;
        if b'0' <= c && c <= b'9' {
            res += CODE_DIGITS - b'0' as i32;
        } else if b'a' <= c && c <= b'z' {
            res += CODE_LOWERCASE - b'a' as i32;
        } else if b'A' <= c && c <= b'Z' {
            res += CODE_UPPERCASE - b'A' as i32;
        } else {
            continue;
        }
        res ^= XOR_MASK;
        res &= AND_MASK;
        if res == CODE_AT {
            putchar(&mut stdout, b'@');
        } else if res == CODE_SHARP {
            putchar(&mut stdout, b'#');
        } else if res < CODE_LOWERCASE {
            putchar(&mut stdout, (res - CODE_DIGITS + b'0' as i32) as u8);
        } else if res < CODE_UPPERCASE {
            putchar(&mut stdout, (res - CODE_LOWERCASE + b'a' as i32) as u8);
        } else {
            putchar(&mut stdout, (res - CODE_UPPERCASE + b'A' as i32) as u8);
        }
    }
}
