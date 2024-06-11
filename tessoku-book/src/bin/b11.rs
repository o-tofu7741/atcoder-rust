#![allow(unused_imports, unused_macros, dead_code)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize;n],
        q: usize,
        x: [usize;q]
    }
    a.sort();
    for i in &x {
        let r = a.binary_search(i);
        let mut ans = 0;
        if r.is_ok() {
            ans = r.unwrap();
        } else {
            ans = r.unwrap_err();
        }
        println!("{}", ans);
    }
}
