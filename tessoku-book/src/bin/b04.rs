#![allow(unused_imports, unused_macros, dead_code)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    println!("{}", (usize::from_str_radix(&(n.to_string()), 2).unwrap()))
}
