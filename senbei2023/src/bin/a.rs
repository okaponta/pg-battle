use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i64;n],
    }
    let mut is_plus = true;
    for a in a {
        if a == 0 {
            println!("0");
            return;
        }
        if a < 0 {
            is_plus = !is_plus;
        }
    }
    println!("{}", if is_plus { "+" } else { "-" });
}
