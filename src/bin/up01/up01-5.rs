use std::error::Error;
use std::io::{self, BufRead, Write};
use std::process::ExitCode;

const MIN_N: usize = 1;
const MAX_N: usize = 9;

fn print_permutations(
    n: usize,
    c: usize,
    curr: &mut [u8],
    used: &mut [bool],
) -> Result<(), io::Error> {
    if c >= n {
        io::stdout().lock().write(&curr[..n])?;
        println!();
        return Ok(());
    }
    for i in 1..=n {
        if used[i - 1] {
            continue;
        }
        used[i - 1] = true;
        curr[c] = i as u8 + b'0';
        print_permutations(n, c + 1, curr, used)?;
        used[i - 1] = false;
    }
    Ok(())
}

fn try_main() -> Result<(), Box<dyn Error>> {
    let mut n = String::new();
    io::stdin().lock().read_line(&mut n)?;
    let n: usize = n.trim().parse()?;
    if n < MIN_N || n > MAX_N {
        return Err(format!("parameter N out of range [{MIN_N}; {MAX_N}]").into());
    }
    let mut curr = [b'\0'; MAX_N];
    let mut used = [false; MAX_N];
    print_permutations(n, 0, &mut curr, &mut used)?;
    Ok(())
}

fn main() -> ExitCode {
    match try_main() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}
