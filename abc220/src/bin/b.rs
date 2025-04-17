use proconio::*;

fn main() {
    input! {
        k: i64,
        mut a: i64,
        mut b: i64
    }

    let mut ab = 0;
    let mut digit: u32 = 0;
    while a > 0 {
        let last = a % 10;
        a /= 10;
        ab += last * k.pow(digit);
        digit += 1;
    }

    let mut bb = 0;
    let mut digit: u32 = 0;
    while b > 0 {
        let last = b % 10;
        b /= 10;
        bb += last * k.pow(digit);
        digit += 1;
    }

    println!("{}", ab * bb);
}
