use std::io;
use std::io::{BufRead, Write};

const MAX_SIZE: usize = 9;

fn print_permutations(n: usize, c: usize, curr: &mut [u8], used: &mut [bool]) {
    if c >= n {
        io::stdout().lock().write(&curr[..n]).expect("Output error");
        println!();
        return;
    }
    for i in 1..=n {
        if used[i - 1] {
            continue;
        }
        used[i - 1] = true;
        curr[c] = i as u8 + b'0';
        print_permutations(n, c + 1, curr, used);
        used[i - 1] = false;
    }
}

fn main() {
    let mut n = String::new();
    io::stdin().lock().read_line(&mut n).expect("Input error");
    let n: usize = n.trim().parse().expect("Input format error");
    assert!(n > 0 && n < 10, "Input format error");
    let mut curr = [b'\0'; MAX_SIZE];
    let mut used = [false; MAX_SIZE];
    print_permutations(n, 0, &mut curr, &mut used);
}
