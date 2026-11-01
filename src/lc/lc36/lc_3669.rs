// https://leetcode.com/problems/balanced-k-factor-decomposition/
// 3669. Balanced K-Factor Decomposition
pub struct Solution;
impl Solution {
    fn track(n: i32, k: i32, s: i32, v: &mut Vec<i32>, max: &mut i32, res: &mut Vec<i32>) {
        if n < s {
            return;
        }
        if k == 1 {
            if n >= s {
                v.push(n);
                if n - v[0] < *max {
                    *max = n - v[0];
                    *res = v.clone();
                }
                v.pop();
            }
            return;
        }
        for i in s..=n {
            if n % i == 0 {
                v.push(i);
                Self::track(n / i, k - 1, i, v, max, res);
                v.pop();
            }
        }
    }
    pub fn min_difference(n: i32, k: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut max = i32::MAX;
        let mut v = vec![];
        Self::track(n, k, 1, &mut v, &mut max, &mut res);
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_difference() {
        assert_eq!(Solution::min_difference(100, 2), vec![10, 10]);
        assert_eq!(Solution::min_difference(44, 3), vec![2, 2, 11]);
    }
}
