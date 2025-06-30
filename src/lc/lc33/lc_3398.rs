// https://leetcode.com/problems/smallest-substring-with-identical-characters-i/
// 3398. Smallest Substring With Identical Characters I
pub struct Solution;
impl Solution {
    pub fn min_length(s: String, num_ops: i32) -> i32 {
        super::lc_3399::Solution::min_length(s, num_ops)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_length() {
        assert_eq!(Solution::min_length("000001".to_string(), 1), 2);
        assert_eq!(Solution::min_length("0000".to_string(), 2), 1);
        assert_eq!(Solution::min_length("0101".to_string(), 0), 1);
    }
}
