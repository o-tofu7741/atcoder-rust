#![allow(unused_imports, unused_macros, dead_code)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize,usize); n]
    }
    let mut times = vec![0; t + 1];
    for (l, r) in lr {
        times[l] += 1;
        times[r] -= 1;
    }
    let s: Vec<isize> = times
        .iter()
        .scan(0, |sum, x| {
            *sum += x;
            Some(*sum)
        })
        .collect();
    for i in 0..t {
        println!("{}", s[i])
    }
}
