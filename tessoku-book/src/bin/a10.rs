#![allow(unused_imports, unused_macros, dead_code)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n],
        d:usize,
        lr:[(usize,usize);d],
    }
    let mut s = vec![vec![0; n + 2]; 2];
    for i in 1..=n {
        s[0][i] = s[0][i - 1].max(a[i - 1]);
        s[1][n - i + 1] = s[1][n - i + 2].max(a[n - i]);
    }
    for (l, r) in lr {
        println!("{}", s[0][l - 1].max(s[1][r + 1]))
    }
}
