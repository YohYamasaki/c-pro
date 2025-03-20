use proconio::*;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: i32,
        a: [i32; n]
    }

    let mut prime_factor_set: HashSet<i32> = HashSet::new();
    for el in a {
        for pf in prime_factorize(el) {
            prime_factor_set.insert(pf);
        }
    }

    let prime_factors = Vec::from_iter(prime_factor_set.iter());
    // let mut prime_numbers = vec![];
    let mut numbers: Vec<_> = (1..m + 1).collect();
    for pf in prime_factors {
        numbers = numbers.iter().filter(|&x| x % pf != 0).cloned().collect();
    }

    println!("{}", numbers.len());
    for ans in numbers {
        println!("{ans}");
    }
}

fn prime_factorize(mut num: i32) -> Vec<i32> {
    let mut res = vec![];
    let mut i = 2;
    while i * i <= num {
        while num % i == 0 {
            res.push(i);
            num /= i;
        }
        i += 1;
    }
    if num > 2 {
        res.push(num);
    }
    res
}

// 6 1 5
// 1 2 3 5

// 1 2 3 4 5 6 7 8 9 10 11 12
// - x x x x x - x x  x  -  x

// 1. 素因数分解したリストを作る
// 2. M+1長のリストを用意してエラストテネスの篩
// 3. 引っ掛からなかったものが答え
