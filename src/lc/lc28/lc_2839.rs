// https://leetcode.com/problems/check-if-strings-can-be-made-equal-with-operations-i/
// 2839. Check if Strings Can Be Made Equal With Operations I
pub struct Solution;
impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        (s1[0] == s2[0] && s1[2] == s2[2] || s1[0] == s2[2] && s1[2] == s2[0])
            && (s1[1] == s2[1] && s1[3] == s2[3] || s1[1] == s2[3] && s1[3] == s2[1])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_be_equal() {
        assert_eq!(Solution::can_be_equal("abcd".to_string(), "cdab".to_string()), true);
        assert_eq!(Solution::can_be_equal("abcd".to_string(), "dacb".to_string()), false);
    }
}
