use proconio::*;

fn main() {
    input! {
        n: i64
    }

    let mut ans = 0;
    for i in 0..60 {
        if 2_i64.pow(i) <= n {
            ans = i;
        }
    }
    println!("{ans}")
}
