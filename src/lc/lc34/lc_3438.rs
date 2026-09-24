// https://leetcode.com/problems/find-valid-pair-of-adjacent-digits-in-string/
// 3438. Find Valid Pair of Adjacent Digits in String
pub struct Solution;
impl Solution {
    pub fn find_valid_pair(s: String) -> String {
        let s = s.as_bytes();
        let mut cnt = [0; 10];
        for &c in s {
            cnt[(c - b'0') as usize] += 1;
        }
        for i in 1..s.len() {
            if s[i - 1] != s[i]
                && cnt[(s[i - 1] - b'0') as usize] == s[i - 1] - b'0'
                && cnt[(s[i] - b'0') as usize] == s[i] - b'0'
            {
                return format!("{}{}", s[i - 1] as char, s[i] as char);
            }
        }
        "".to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_valid_pair() {
        assert_eq!(Solution::find_valid_pair("2523533".to_string()), "23".to_string());
        assert_eq!(Solution::find_valid_pair("221".to_string()), "21".to_string());
        assert_eq!(Solution::find_valid_pair("22".to_string()), "".to_string());
    }
}
