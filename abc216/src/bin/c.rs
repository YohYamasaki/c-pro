use proconio::*;

fn main() {
    input! {
        mut n: i64
    }

    let mut ans = vec![];
    while n > 0 {
        if n % 2 == 0 {
            n /= 2;
            ans.push('B');
        } else {
            n -= 1;
            ans.push('A');
        }
    }

    ans.reverse();
    for a in ans {
        print!("{}", a);
    }
    println!("");
}
