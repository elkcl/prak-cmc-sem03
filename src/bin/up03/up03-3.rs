use std::env;
use std::error::Error;
use std::process::ExitCode;

fn try_main() -> Result<(), Box<dyn Error>> {
    const ROUND_COEFFICIENT: i64 = 10_000;
    const PERCENT: i64 = 100;
    const BASE: i64 = 10;
    const LAST_DIGIT_ROUND: i64 = 5;

    let mut args = env::args_os().skip(1);
    let init = args.next().ok_or("no initial price given")?;
    let init = init.to_str().ok_or("invalid UTF-8")?;
    let init: f64 = init.parse()?;
    let mut init: i64 = (init * (ROUND_COEFFICIENT as f64)) as i64;

    for curr in args {
        let curr = curr.to_str().ok_or("invalid UTF-8")?;
        let curr: f64 = curr.parse()?;
        init *= PERCENT;
        init *= PERCENT * ROUND_COEFFICIENT + (curr * (ROUND_COEFFICIENT as f64)) as i64;
        init /= ROUND_COEFFICIENT * PERCENT * PERCENT / BASE;
        if init % BASE >= LAST_DIGIT_ROUND {
            init += BASE;
        }
        init /= BASE;
    }

    let ans_int = init / ROUND_COEFFICIENT;
    let ans_frac = init % ROUND_COEFFICIENT;
    println!("{}.{:04}", ans_int, ans_frac);

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
