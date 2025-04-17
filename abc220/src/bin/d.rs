use proconio::*;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }
    let mut dp: Vec<Vec<i64>> = vec![vec![0; 10]; n];

    dp[0][a[0]] = 1;
    let m = 998244353;

    for i in 0..n - 1 {
        for j in 0..10 {
            let cnt = dp[i][j];
            if cnt == 0 {
                continue;
            }
            let x = a[i + 1] % 10;
            dp[i + 1][(j + x) % 10] = (dp[i + 1][(j + x) % 10] + cnt) % m;
            dp[i + 1][(j * x) % 10] = (dp[i + 1][(j * x) % 10] + cnt) % m;
        }
    }

    for val in &dp[n - 1] {
        println!("{val}");
    }
}
