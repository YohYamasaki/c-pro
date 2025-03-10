use num::abs;
use proconio::*;
use std::cmp::*;

fn main() {
    input! {
        n: i32,
        m: i32,
        mut a: [i32; n],
        mut b: [i32; m],
    }
    a.sort();
    b.sort();

    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut mn = 1001001001;
    while (i as i32) < n && (j as i32) < m {
        let diff = abs(a[i] - b[j]);
        mn = mn.min(diff);
        if a[i] > b[j] {
            j += 1;
        } else {
            i += 1;
        }
    }

    println!("{mn}");
}
