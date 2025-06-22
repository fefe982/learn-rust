// https://leetcode.com/problems/count-binary-substrings/
// 696. Count Binary Substrings
pub struct Solution;
impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut last_c = 0;
        let mut last = '.';
        let mut cur = 0;
        let mut sum = 0;
        for c in s.chars() {
            if c == last {
                cur += 1;
            } else {
                sum += last_c.min(cur);
                last_c = cur;
                cur = 1;
                last = c;
            }
        }
        sum + last_c.min(cur)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_binary_substrings() {
        assert_eq!(Solution::count_binary_substrings("00110011".to_string()), 6);
        assert_eq!(Solution::count_binary_substrings("10101".to_string()), 4);
    }
}
