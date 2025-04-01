use proconio::*;

fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
                t: String
    }

    let mut ans: Vec<&String> = vec![];

    for c in t.chars() {
        if c == '1' {
            ans.push(&s1);
        } else if c == '2' {
            ans.push(&s2);
        } else {
            ans.push(&s3);
        }
    }

    for i in ans {
        print!("{i}");
    }
    println!("");
}
