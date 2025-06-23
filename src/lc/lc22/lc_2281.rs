// https://leetcode.com/problems/sum-of-total-strength-of-wizards/
//2281. Sum of Total Strength of Wizards
use std::iter;
pub struct Solution;
const M: i64 = 1_000_000_007;
impl Solution {
    fn add(x: i64, y: i64) -> i64 {
        (x + y) % M
    }
    fn sub(x: i64, y: i64) -> i64 {
        (x - y + M) % M
    }
    fn mul(x: i64, y: i64) -> i64 {
        (x * y) % M
    }
    pub fn total_strength(strength: Vec<i32>) -> i32 {
        let len = strength.len();
        let mut p = vec![0i64; len + 3];
        let mut sum = 0i64;
        for (idx, a) in strength.iter().enumerate() {
            sum = Self::add(sum, *a as i64);
            p[idx + 2] = Self::add(sum, p[idx + 1]);
        }
        p[len + 2] = Self::add(sum, p[len + 1]);

        let mut s = vec![0usize];
        sum = 0i64;
        for (idx, a) in strength.iter().chain(iter::once(&0i32)).enumerate() {
            while s.len() > 1 && strength[s.last().unwrap() - 1] > *a {
                let r = idx + 1;
                let i = s.pop().unwrap();
                let l = *s.last().unwrap();
                let ai = strength[i - 1] as i64;
                sum = Self::add(
                    sum,
                    Self::mul(
                        ai,
                        Self::sub(
                            Self::mul((i - l) as i64, Self::sub(p[r], p[i])),
                            Self::mul((r - i) as i64, Self::sub(p[i], p[l])),
                        ),
                    ),
                )
            }
            s.push(idx + 1)
        }
        sum as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn total_strength() {
        assert_eq!(Solution::total_strength(vec![1, 3, 1, 2]), 44);
        assert_eq!(Solution::total_strength(vec![5, 4, 6]), 213);
    }
}
