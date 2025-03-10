use proconio::*;

fn main() {
    input! {
        n: i32,
        a: [i32; n]
    }

    let mut arr = Vec::new();
    for (i, v) in a.iter().enumerate() {
        arr.push(vec![*v, i as i32]);
    }

    arr.sort();
    arr.reverse();
    let ans = &arr[1][1];
    println!("{}", ans + 1);
}
