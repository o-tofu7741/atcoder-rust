#![allow(unused_imports, unused_macros, dead_code)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a:[usize;n],
        lr:[(usize,usize);q],
    }
    let mut s: Vec<usize> = vec![0];
    for i in &a {
        s.push(s.last().unwrap() + i);
    }
    // println!("{:?}", &s);
    for (l, r) in lr {
        println!("{}", s[r] - s[l-1])
    }
}
