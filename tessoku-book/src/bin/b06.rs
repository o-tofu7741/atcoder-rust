#![allow(unused_imports, unused_macros, dead_code)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n],
        q:usize,
        lr:[(usize,usize);q],
    }
    let mut s = vec![0];
    for i in a {
        s.push(s.last().unwrap() + if i == 1 { 1 } else { -1 });
    }
    for (l, r) in lr {
        println!(
            "{}",
            if 0 < (s[r] - s[l - 1]) {
                "win"
            } else if 0 == (s[r] - s[l - 1]) {
                "draw"
            } else {
                "lose"
            }
        )
    }
}
