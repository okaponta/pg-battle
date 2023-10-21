use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n],
    }
    if k == 0 {
        println!("1");
        return;
    }
    let mut fact = vec![0; n + 1];
    let mut inv = vec![0; n + 1];
    let mut fact_inv = vec![0; n + 1];
    fact[0] = 1;
    fact[1] = 1;
    inv[1] = 1;
    fact_inv[0] = 1;
    fact_inv[1] = 1;
    for i in 2..(n + 1) {
        fact[i] = fact[i - 1] * i % MOD;
        inv[i] = MOD - inv[MOD % i] * (MOD / i) % MOD;
        fact_inv[i] = fact_inv[i - 1] * inv[i] % MOD;
    }
    let mut mv = vec![HashMap::new(); k];
    let mut count = vec![0; k];
    for i in 0..n {
        count[i % k] += 1;
        *mv[i % k].entry(a[i]).or_insert(0) += 1;
    }
    let mut ans = ModInt::one();
    for i in 0..k {
        ans = ans * ModInt::new(fact[count[i]]);
        for (_, v) in &mv[i] {
            ans = ans * ModInt::new(fact_inv[*v]);
        }
    }
    println!("{}", ans.val);
}

const MOD: usize = 998244353;

#[derive(Clone, Copy)]
pub struct ModInt {
    val: usize,
}

impl ModInt {
    pub const fn zero() -> Self {
        Self { val: 0 }
    }

    pub const fn one() -> Self {
        Self { val: 1 }
    }

    pub fn new(i: usize) -> Self {
        ModInt { val: i % MOD }
    }

    pub fn inv(&self) -> Self {
        let mut a = self.val as i64;
        let mut b = MOD as i64;
        let modulo = MOD as i64;
        let mut u = 1;
        let mut v = 0;
        while b > 0 {
            let t = a / b;
            a -= t * b;
            std::mem::swap(&mut a, &mut b);
            u -= t * v;
            std::mem::swap(&mut u, &mut v);
        }
        u %= modulo;
        if u < 0 {
            u += modulo;
        }
        ModInt { val: u as usize }
    }

    pub fn pow(&self, mut n: usize) -> Self {
        let mut x = self.val;
        // let mut ret = if x == 0 { 0 } else { 1 };
        let mut ret = 1;
        while n > 0 {
            if n & 1 == 1 {
                ret = ret * x % MOD;
            }
            x = x * x % MOD;
            n >>= 1;
        }
        ModInt { val: ret }
    }
}

impl std::ops::Add for ModInt {
    type Output = ModInt;
    fn add(self, other: Self) -> Self {
        ModInt::new(self.val + other.val)
    }
}

impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl std::ops::Sub for ModInt {
    type Output = ModInt;
    fn sub(self, other: Self) -> Self {
        ModInt::new(MOD + self.val - other.val)
    }
}

impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl std::ops::Mul for ModInt {
    type Output = ModInt;
    fn mul(self, other: Self) -> Self {
        ModInt::new(self.val * other.val)
    }
}

impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}
