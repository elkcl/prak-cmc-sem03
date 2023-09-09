use std::io;
use std::io::Write;

fn putchar(h: &mut impl Write, c: u8) {
    let buf: [u8; 1] = [c];
    h.write(&buf).expect("Output error");
}


fn divmod(c: i32, mut a: i32, n: i32) -> i32 {
    let mut p = n - 2;
    let mut res = 1;
    while p > 0 {
        if p % 2 != 0 {
            res *= a;
            res %= n;
        }
        a *= a;
        a %= n;
        p /= 2;
    }
    (res * c) % n
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Input error");
    let n: i32 = n.trim().parse().expect("Input format error");
    
    let mut stdout = io::stdout().lock();
    for c in 0..n {
        for a in 1..n {
            write!(&mut stdout, "{} ", divmod(c, a, n)).expect("Output error");
        }
        putchar(&mut stdout, b'\n');
    }
}
