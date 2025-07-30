// https://leetcode.com/problems/maximum-or/
// 2680. Maximum OR
pub struct Solution;
impl Solution {
    pub fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
        let mut all = 0;
        let mut multi = 0;
        for &n in &nums {
            multi |= all & n;
            all |= n;
        }
        let mut max = 0;
        let multi = multi as i64;
        let all = all as i64;
        for n in nums {
            let n = n as i64;
            max = max.max((all ^ n) | multi | (n << k));
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_or() {
        assert_eq!(Solution::maximum_or(vec![12, 9], 1), 30);
        assert_eq!(Solution::maximum_or(vec![8, 1, 2], 2), 35);
    }
}
