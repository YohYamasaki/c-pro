use proconio::*;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        s: [i32; n],
        t: [i32; n]
    }

    let mut memo = t.clone();
    for i in 0..n * 2 {
        let pi = (n + i - 1) % n;
        let ci = i % n;
        memo[ci] = min(memo[ci], memo[pi] + s[pi]);
    }

    for ans in memo {
        println!("{ans}");
    }
}
