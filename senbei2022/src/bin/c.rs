use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut q = VecDeque::new();
    for a in a.into_iter().rev() {
        if a == 1 {
            let mut tmp = 1;
            while let Some(top) = q.pop_front() {
                if tmp + 1 == top {
                    tmp += 1;
                } else {
                    q.push_front(top);
                    break;
                }
            }
        } else {
            q.push_front(a);
        }
    }
    println!("{}", q.into_iter().sum::<usize>());
}
