use num::range;
use proconio::*;

fn main() {
    input! {
        x: String,
    }

    let d: Vec<i32> = x.chars().map(|c| c as i32 - '0' as i32).collect();

    // x1 == x2 == x3 == x4はNG
    if d[0] == d[1] && d[1] == d[2] && d[2] == d[3] {
        println!("Weak");
        return;
    }

    let mut is_weak = true;
    for i in range(0, 3) {
        if (d[i] + 1) % 10 != (d[i + 1] % 10) {
            is_weak = false;
        }
    }

    if is_weak {
        println!("Weak");
        return;
    }

    println!("Strong");
}
