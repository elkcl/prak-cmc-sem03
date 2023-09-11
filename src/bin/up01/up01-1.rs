use std::io;

fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("no input");
    let x: f64 = x.trim().parse().expect("not a double");
    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("no input");
    let y: f64 = y.trim().parse().expect("not a double");

    if x < 2.0 || x > 5.0 || y < 1.0 || y > 7.0 {
        println!("0");
        return;
    }
    if x >= 3.0 && x <= 5.0 && y >= 1.0 && y <= 3.0 && (y - 1.0) < (x - 3.0) {
        println!("0");
        return;
    }
    println!("1");
}
