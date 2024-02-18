use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        m: usize,
        n: usize,
        k: usize,
        area: [Chars; m],
        queries: [(usize, usize, usize, usize); k],
    }

    let mut j_acc = vec![vec![0; n + 1]; m];
    let mut o_acc = vec![vec![0; n + 1]; m];

    for i in 0..m {
        for j in 0..n {
            j_acc[i][j + 1] = j_acc[i][j] + if area[i][j] == 'J' { 1 } else { 0 };
            o_acc[i][j + 1] = o_acc[i][j] + if area[i][j] == 'O' { 1 } else { 0 };
        }
    }

    for query in queries {
        let (a, b, c, d) = query;
        let mut ans1 = 0;
        let mut ans2 = 0;
        for m in a - 1..c {
            ans1 += j_acc[m][d] - j_acc[m][b - 1];
            ans2 += o_acc[m][d] - o_acc[m][b - 1];
        }
        println!(
            "{} {} {}",
            ans1,
            ans2,
            (c - a + 1) * (d - b + 1) - ans1 - ans2
        );
    }
}
