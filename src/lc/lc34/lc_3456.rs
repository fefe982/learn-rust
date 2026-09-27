// https://leetcode.com/problems/find-special-substring-of-length-k/
// 3456. Find Special Substring of Length K
pub struct Solution;
impl Solution {
    pub fn has_special_substring(s: String, k: i32) -> bool {
        let mut last = '.';
        let mut cnt = 0;
        for c in s.chars() {
            if c == last {
                cnt += 1;
            } else {
                if cnt == k {
                    return true;
                }
                last = c;
                cnt = 1;
            }
        }
        cnt == k
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn has_special_substring() {
        assert_eq!(Solution::has_special_substring("aaabaaa".to_string(), 3), true);
        assert_eq!(Solution::has_special_substring("abc".to_string(), 2), false);
    }
}
