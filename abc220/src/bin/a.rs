use proconio::*;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }

    let mut d = c;
    while d <= 1000 {
        if a <= d && d <= b {
            println!("{d}");
            return;
        }
        d += c;
    }
    println!("-1");
}
