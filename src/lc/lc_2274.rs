// https://leetcode.cn/problems/maximum-consecutive-floors-without-special-floors/
// 2274. Maximum Consecutive Floors Without Special Floors
pub struct Solution;
impl Solution {
    pub fn max_consecutive(bottom: i32, top: i32, special: Vec<i32>) -> i32 {
        let mut special = special;
        special.push(bottom - 1);
        special.push(top + 1);
        special.sort_unstable();
        let mut m = 0;
        for i in 1..special.len() {
            m = m.max(special[i] - special[i - 1] - 1);
        }
        m
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_consecutive() {
        assert_eq!(Solution::max_consecutive(2, 9, vec![4, 6]), 3);
        assert_eq!(Solution::max_consecutive(6, 8, vec![7, 6, 8]), 0);
    }
}
