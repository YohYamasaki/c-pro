use proconio::*;

fn main() {
    input! {
        s: String
    }
    let parts: Vec<&str> = s.split('.').collect();
    let xs = parts[0];
    let ys = parts[1];
    let x = xs.parse::<i32>().unwrap();
    let y = ys.parse::<i32>().unwrap();
    if y <= 2 {
        println!("{x}-");
    } else if y <= 6 {
        println!("{x}");
    } else {
        println!("{x}+");
    }
}
