#![allow(unused_imports, unused_macros, dead_code)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut ans = 0;
    for i in 1..=(k - 2) {
        for j in 1..=(k - i - 1) {
            if 1 <= i && i <= n && 1 <= j && j <= n && 1 <= (k - i - j) && (k - i - j) <= n {
                ans += 1;
            }
        }
    }
    println!("{}", ans)
}
