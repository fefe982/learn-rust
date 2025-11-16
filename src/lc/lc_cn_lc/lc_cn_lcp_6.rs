// https://leetcode.cn/problems/na-ying-bi/
// LCP 06. 拿硬币
pub struct Solution;
impl Solution {
    pub fn min_count(coins: Vec<i32>) -> i32 {
        coins.iter().fold(0, |s, &c| s + (c + 1) / 2)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_count() {
        assert_eq!(Solution::min_count(vec![4, 2, 1]), 4);
        assert_eq!(Solution::min_count(vec![2, 3, 10]), 8);
    }
}
