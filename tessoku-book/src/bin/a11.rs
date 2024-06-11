#![allow(unused_imports, unused_macros, dead_code)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize;n]
    }
    println!("{}", a.binary_search(&x).unwrap() + 1);
}
