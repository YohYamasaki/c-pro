use proconio::*;

fn main() {
    input! {
        n: i32,
        names: [[String;2];n]
    }
    let mut ans = "No";
    for (i, name1) in names.iter().enumerate() {
        for (j, name2) in names.iter().enumerate() {
            if i != j && name1[0] == name2[0] && name1[1] == name2[1] {
                ans = "Yes";
            }
        }
    }
    println!("{ans}");
}
