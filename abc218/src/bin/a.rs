use proconio::*;

fn main() {
    input! {
        n: usize,
        s: String
    }

    for (i, c) in s.chars().enumerate() {
        if i == n - 1 {
            if c == 'o' {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
