// https://leetcode.com/problems/count-number-of-trapezoids-i/
// 3623. Count Number of Trapezoids I
pub struct Solution;
impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        for p in points {
            *cnt.entry(p[1]).or_insert(0) += 1;
        }
        let mut ans = 0;
        const MOD: i64 = 1_000_000_007;
        let mut sum = 0;
        for (_, c) in cnt {
            let ic = c as i64;
            let edge = (ic * (ic - 1) / 2) % MOD;
            ans = (ans + sum * edge) % MOD;
            sum = (sum + edge) % MOD;
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_trapezoids() {
        assert_eq!(
            Solution::count_trapezoids(vec_vec![[1, 0], [2, 0], [3, 0], [2, 2], [3, 2]]),
            3
        );
        assert_eq!(Solution::count_trapezoids(vec_vec![[0, 0], [1, 0], [0, 1], [2, 1]]), 1);
    }
}
