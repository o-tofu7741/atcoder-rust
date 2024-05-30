#![allow(unused_imports, unused_macros, dead_code)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h:usize,
        w:usize,
        x:[[isize;w];h],
        q:usize,
        abcd:[(usize,usize,usize,usize);q]
    }
    let mut s: Vec<Vec<isize>> = vec![vec![0; w + 1]; h + 1];
    for i in 1..=h {
        for j in 1..=w {
            s[i][j] = x[i - 1][j - 1] + s[i][j - 1];
        }
        for j in 1..=w {
            s[i][j] += s[i - 1][j];
        }
    }
    for (a, b, c, d) in abcd {
        println!("{}", s[c][d] - s[a - 1][d] - s[c][b - 1] + s[a - 1][b - 1])
    }
}
