// https://leetcode.com/problems/find-unique-binary-string/
// 1980. Find Unique Binary String
pub struct Solution;
impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut res = String::new();
        for (i, n) in nums.into_iter().enumerate() {
            if n.as_bytes()[i] == b'0' {
                res.push('1');
            } else {
                res.push('0');
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_different_binary_string() {
        assert_eq!(Solution::find_different_binary_string(vec_str!["01", "10"]), "11");
        assert_eq!(Solution::find_different_binary_string(vec_str!["00", "01"]), "10");
        assert_eq!(
            Solution::find_different_binary_string(vec_str!["111", "011", "001"]),
            "000"
        );
    }
}
