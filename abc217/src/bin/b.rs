use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String
    }

    let mut st = HashSet::from([
        String::from("ABC"),
        String::from("ARC"),
        String::from("AGC"),
        String::from("AHC"),
    ]);
    st.remove(&s1);
    st.remove(&s2);
    st.remove(&s3);

    for i in st {
        println!("{i}");
    }
}
