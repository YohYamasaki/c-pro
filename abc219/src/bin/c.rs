use std::collections::HashMap;

use proconio::*;

fn main() {
    input! {
        x: String,
        n: usize,
        s: [String; n]
    }

    let mut char_to_i: HashMap<char, usize> = HashMap::new();
    let mut i_to_char: HashMap<usize, char> = HashMap::new();
    for (i, c) in x.chars().enumerate() {
        char_to_i.insert(c, i);
        i_to_char.insert(i, c);
    }

    let mut words_index: Vec<Vec<usize>> = vec![];

    for word in s {
        let mut si: Vec<usize> = vec![];
        for c in word.chars() {
            si.push(*char_to_i.get(&c).unwrap());
        }
        words_index.push(si);
    }

    words_index.sort();

    for w in words_index {
        for c in w {
            print!("{}", i_to_char.get(&c).unwrap());
        }
        println!("");
    }
}
