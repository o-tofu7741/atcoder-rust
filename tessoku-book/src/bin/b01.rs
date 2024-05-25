#![allow(unused_imports, unused_macros, dead_code)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        a:usize,
        b:usize,
    }
    println!("{}", a + b);
}
