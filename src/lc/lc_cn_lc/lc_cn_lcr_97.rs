// https://leetcode.cn/problems/21dk04/
// LCR 097. 不同的子序列
pub struct Solution;
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        super::super::lc01::lc_115::Solution::num_distinct(s, t)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_distinct() {
        assert_eq!(
            Solution::num_distinct(String::from("rabbbit"), String::from("rabbit")),
            3
        );
        assert_eq!(Solution::num_distinct(String::from("babgbag"), String::from("bag")), 5);
    }
}
