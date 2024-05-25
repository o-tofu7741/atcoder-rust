#![allow(unused_imports, unused_macros, dead_code)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize;n]
    }
    let mut ans = "No";
    for i in &a {
        if i == &x {
            ans = "Yes";
        }
    }
    println!("{}", ans)
}
