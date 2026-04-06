// https://leetcode.com/problems/replace-all-digits-with-characters/
// 1844. Replace All Digits with Characters
pub struct Solution;
impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut ans = s.into_bytes();
        for i in (1..ans.len()).step_by(2) {
            ans[i] += ans[i - 1] - b'0';
        }
        String::from_utf8(ans).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn replace_digits() {
        assert_eq!(Solution::replace_digits("a1c1e1".to_string()), "abcdef");
        assert_eq!(Solution::replace_digits("a1b2c3d4e".to_string()), "abbdcfdhe");
    }
}
