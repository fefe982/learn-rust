// https://leetcode.com/problems/binary-trees-with-factors/
// 823. Binary Trees With Factors
pub struct Solution;
const MOD: i64 = 1000000007;
#[derive(Copy, Clone)]
struct IMod {
    i: i64,
}
impl IMod {
    fn new(i: i32) -> Self {
        Self { i: i as i64 }
    }
    fn from_i64(i: i64) -> Self {
        Self { i }
    }
    fn val(self) -> i32 {
        self.i as i32
    }
}
impl std::ops::Add<IMod> for IMod {
    type Output = IMod;
    fn add(self, rhs: IMod) -> IMod {
        IMod::from_i64((self.i + rhs.i) % MOD)
    }
}
impl std::ops::AddAssign<IMod> for IMod {
    fn add_assign(&mut self, rhs: IMod) {
        self.i = (self.i + rhs.i) % MOD;
    }
}
impl std::ops::Mul<IMod> for IMod {
    type Output = IMod;
    fn mul(self, rhs: IMod) -> Self::Output {
        IMod::from_i64((self.i * rhs.i) % MOD)
    }
}
impl std::ops::MulAssign<i32> for IMod {
    fn mul_assign(&mut self, rhs: i32) {
        self.i = (self.i * rhs as i64) % MOD;
    }
}
impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        arr.sort_unstable();
        let mut cnt = std::collections::BTreeMap::<i32, IMod>::new();
        let mut total = IMod::new(0);
        for n in arr {
            let mut t = IMod::new(1);
            for (&f, &c) in &cnt {
                if f * f > n {
                    break;
                }
                if n % f == 0 {
                    let nf = n / f;
                    if let Some(&cnf) = cnt.get(&nf) {
                        let mut m = c * cnf;
                        if f != nf {
                            m *= 2;
                        }
                        t += m;
                    }
                }
            }
            total += t;
            cnt.insert(n, t);
        }
        total.val()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_factored_binary_trees() {
        assert_eq!(Solution::num_factored_binary_trees(vec![2, 4]), 3);
        assert_eq!(Solution::num_factored_binary_trees(vec![2, 4, 5, 10]), 7);
    }
}
