use std::collections::HashSet;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {
        s: String,
        k: usize
    }

    let per = s.chars().permutations(s.len());
    let mut st: HashSet<String> = HashSet::new();
    for p in per {
        st.insert(p.iter().collect::<String>());
    }

    let mut res: Vec<_> = st.into_iter().collect();
    res.sort();
    println!("{}", res[k - 1]);
}
