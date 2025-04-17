use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        x: i64
    }

    let mut s = 0;
    for num in &a {
        s += num;
    }

    let p = x / s;
    let mut sumb = p * s;
    let mut ans = p * n as i64;
    for num in &a {
        sumb += *num;
        ans += 1;
        if sumb > x {
            println!("{}", ans);
            break;
        }
    }
}
