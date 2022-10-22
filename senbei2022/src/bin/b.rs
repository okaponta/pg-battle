use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[usize;3*n],
    }
    let mut a_idx = vec![];
    for i in 0..3 * n {
        a_idx.push((a[i], i));
    }
    a_idx.sort();
    let mut ans = vec![];
    for i in n..2 * n {
        ans.push(a_idx[i].1);
    }
    ans.sort();
    for a in ans {
        println!("{}", a + 1);
    }
}
