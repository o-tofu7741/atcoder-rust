#![allow(unused_imports, unused_macros, dead_code)]
use std::vec;

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize,usize); n],
    }
    let mut days = vec![0; d + 1];
    for (l, r) in lr {
        days[l - 1] += 1;
        days[r] -= 1;
    }
    let s: Vec<isize> = days
        .iter()
        .scan(0, |sum, x| {
            *sum += x;
            Some(*sum)
        })
        .collect();
    for i in 0..d {
        println!("{}", s[i])
    }
}
