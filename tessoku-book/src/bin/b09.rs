#![allow(unused_imports, unused_macros, dead_code)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        abcd: [(usize,usize,usize,usize);n]
    }
    let s_size = 1500;
    let mut s = vec![vec![0; s_size + 2]; s_size + 2];
    for (a, b, c, d) in abcd {
        s[a][b] += 1;
        s[a][d] -= 1;
        s[c][b] -= 1;
        s[c][d] += 1;
    }
    for i in 0..=s_size {
        for j in 1..=s_size {
            s[i][j] += s[i][j - 1];
        }
    }

    for i in 1..=s_size {
        for j in 0..=s_size {
            s[i][j] += s[i - 1][j];
        }
    }

    println!(
        "{}",
        s.iter()
            .map(|x| x.iter().filter(|&&y| 0 < y).count())
            .fold(0, |sum, z| sum + z)
    )
}
