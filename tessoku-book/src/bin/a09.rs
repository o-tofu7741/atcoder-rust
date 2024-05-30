#![allow(unused_imports, unused_macros, dead_code)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h:usize,
        w:usize,
        n:usize,
        abcd:[(usize,usize,usize,usize);n]
    }
    let mut s: Vec<Vec<i32>> = vec![vec![0; w + 2]; h + 2];
    for (a, b, c, d) in abcd {
        s[a][b] += 1;
        s[a][d + 1] -= 1;
        s[c + 1][b] -= 1;
        s[c + 1][d + 1] += 1;
    }
    for i in 1..=h {
        for j in 1..=w {
            s[i][j] += s[i][j - 1];
        }
        for j in 1..=w {
            s[i][j] += s[i - 1][j];
        }
    }

    println!(
        "{}",
        s[1..=h]
            .iter()
            .map(|x| x[1..=w]
                .iter()
                .map(|y| y.to_string())
                .collect::<Vec<_>>()
                .join(" "))
            .collect::<Vec<_>>()
            .join("\n")
    )
}
