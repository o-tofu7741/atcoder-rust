use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: Chars,
    }
    println!("{} {:?}", n, a);
}
