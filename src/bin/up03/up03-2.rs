use std::env;
use std::error::Error;
use std::process::ExitCode;

fn try_main() -> Result<(), Box<dyn Error>> {
    let mut sum_pos: i64 = 0;
    let mut sum_neg: i64 = 0;

    for num in env::args_os().skip(1) {
        let num = num.to_str().ok_or("invalid UTF-8")?;
        let num: i32 = num.parse()?;
        if num > 0 {
            let check;
            (sum_pos, check) = sum_pos.overflowing_add(num.into());
            if check {
                return Err("integer overflow".into());
            }
        } else if num < 0 {
            let check;
            (sum_neg, check) = sum_neg.overflowing_add(num.into());
            if check {
                return Err("integer overflow".into());
            }
        }
    }
    println!("{}", sum_pos);
    println!("{}", sum_neg);
    Ok(())
}

fn main() -> ExitCode {
    match try_main() {
        Ok(_) => ExitCode::from(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::from(1)
        }
    }
}
