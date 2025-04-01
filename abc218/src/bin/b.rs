use proconio::*;

fn main() {
    input! {
        p: [u32; 26]
    }
    let mut ans = vec![];
    for n in p {
        ans.push((n as u8 + 96) as char);
    }
    for a in ans {
        print!("{a}");
    }
    println!("");
}
