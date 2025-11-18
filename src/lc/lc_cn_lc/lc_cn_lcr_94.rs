// https://leetcode.cn/problems/omKAoA/
// LCR 094. 分割回文串 II
pub struct Solution;
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        super::super::lc01::lc_132::Solution::min_cut(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_cut() {
        assert_eq!(Solution::min_cut(String::from("aab")), 1);
        assert_eq!(Solution::min_cut(String::from("a")), 0);
        assert_eq!(Solution::min_cut(String::from("ab")), 1);
    }
}
