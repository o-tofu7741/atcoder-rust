#![allow(unused_imports, unused_macros, dead_code)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(usize,usize);n],
        q:usize,
        abcd:[(usize,usize,usize,usize);q]
    }
    let mut s: Vec<Vec<i32>> = vec![vec![0; 1500 + 1]; 1500 + 1];
    for (x, y) in xy {
        s[x][y] += 1;
    }
    for i in 1..=1500 {
        for j in 1..=1500 {
            s[i][j] += s[i][j - 1];
        }
        for j in 1..=1500 {
            s[i][j] += s[i - 1][j];
        }
    }
    for (a, b, c, d) in abcd {
        println!("{}", s[c][d] - s[a - 1][d] - s[c][b - 1] + s[a - 1][b - 1])
    }
}
