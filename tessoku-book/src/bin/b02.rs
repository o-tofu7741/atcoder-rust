#![allow(unused_imports, unused_macros, dead_code)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let mut ans = "No";
    for i in a..=b {
        if 100 % i == 0 {
            ans = "Yes";
        }
    }
    println!("{}", ans)
}
