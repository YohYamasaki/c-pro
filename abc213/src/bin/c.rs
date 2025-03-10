use proconio::*;
use std::collections::HashSet;

fn main() {
    input! {
        _h: i32,
        _w: i32,
        n: i32,
        c: [[i32; 2]; n],
    }

    let a: Vec<i32> = c.iter().map(|e| e[0]).collect();
    let b: Vec<i32> = c.iter().map(|e| e[1]).collect();

    let mut a_comp: Vec<i32> = Vec::from_iter(HashSet::<_>::from_iter(a.clone()));
    a_comp.sort();
    let mut b_comp: Vec<i32> = Vec::from_iter(HashSet::<_>::from_iter(b.clone()));
    b_comp.sort();

    for i in 0..n {
        let a_target = a[i as usize];
        let b_target = b[i as usize];
        let ai = a_comp.binary_search(&a_target).unwrap() + 1;
        let bi = b_comp.binary_search(&b_target).unwrap() + 1;
        println!("{} {}", ai, bi);
    }
}
