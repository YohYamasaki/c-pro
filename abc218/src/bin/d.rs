use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {
        n:usize,
        xy: [[i32;2]; n]
    }
    let x: HashSet<i32> = HashSet::from_iter(xy.iter().map(|el| el[0]));
    // let y: HashSet<i32> = HashSet::from_iter(xy.iter().map(|el| el[1]));
    let mut xmap: HashMap<i32, HashSet<i32>> = HashMap::new();

    for v in xy {
        xmap.entry(v[0]).or_insert_with(HashSet::new).insert(v[1]);
    }

    let mut xc: Vec<Vec<&i32>> = vec![];
    for c in x.iter().combinations(2) {
        xc.push(c);
    }

    let mut cnt = 0;
    for c in xc {
        let xf = c[0];
        let xs = c[1];
        let yfset = xmap.get(xf).unwrap();
        let ysset = xmap.get(xs).unwrap();
        let mut num = 0;
        for yf in yfset {
            if ysset.contains(yf) {
                num += 1;
            }
        }
        cnt += (num * (num - 1)) / 2;
    }

    println!("{cnt}");
}
