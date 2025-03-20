use proconio::*;

fn main() {
    input! {
        s: i32,
        t: i32
    }

    let mut count = 0;
    for a in 0..s + 1 {
        for b in 0..s + 1 {
            for c in 0..s + 1 {
                if a + b + c <= s && a * b * c <= t {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
