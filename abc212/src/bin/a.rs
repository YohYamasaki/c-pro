use proconio::*;

fn main() {
    input! {
        a: i32,
        b: i32
    }
    if 0 < a && b == 0 {
        println!("Gold");
    } else if a == 0 && 0 < b {
        println!("Silver");
    } else {
        println!("Alloy");
    }
}
