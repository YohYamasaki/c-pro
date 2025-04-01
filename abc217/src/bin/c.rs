use proconio::*;

fn main() {
    input! {
        n: i32,
        p: [i32; n]
    }

    let mut ans = vec![0; n as usize];
    for (i, el) in p.iter().enumerate() {
        ans[(el - 1) as usize] = i + 1;
    }
    for i in ans {
        print!("{i} ");
    }
    println!("");
}
