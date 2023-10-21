use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut ans = 0.0;
    let mut dp = vec![0.0; 3];
    dp[0] = 1.0;
    for c in s {
        let mut next = vec![0.0; 3];
        if c == 'A' {
            next = vec![0.0, 1.0, 0.0];
        } else if c == 'B' {
            next[2] = dp[1];
            next[0] = 1.0 - dp[1];
            next[1] = 0.0;
        } else if c == 'C' {
            ans += dp[2];
            next = vec![1.0, 0.0, 0.0];
        } else {
            // ?
            next[1] += 1.0 / 3.0;
            next[2] += dp[1] / 3.0;
            next[0] += (1.0 - dp[1]) / 3.0;
            next[0] += 1.0 / 3.0;
            ans += dp[2] / 3.0;
        }
        dp = next;
    }
    println!("{}", ans);
}
