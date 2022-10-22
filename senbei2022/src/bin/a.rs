use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        q:usize,
        lr:[(Usize1,usize);q],
    }
    let mut imos = vec![0; n + 1];
    for (l, r) in lr {
        imos[l] += 1;
        imos[r] -= 1;
    }
    let mut flip = vec![0; n];
    flip[0] = imos[0];
    for i in 1..n {
        flip[i] = flip[i - 1] + imos[i];
    }
    println!("{}", flip.into_iter().filter(|i| i % 2 == 1).count());
}
