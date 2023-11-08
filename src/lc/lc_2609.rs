// https://leetcode.com/problems/find-the-longest-balanced-substring-of-a-binary-string/
// 2609. Find the Longest Balanced Substring of a Binary String
pub struct Solution;
impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let mut zeros = 0;
        let mut ones = 0;
        let mut last = '0';
        let mut max = 0;
        for c in s.chars().chain(vec!['0']) {
            if c == '1' {
                ones += 1;
            } else {
                if last == '1' {
                    max = max.max(zeros.min(ones) * 2);
                    zeros = 1;
                    ones = 0;
                } else {
                    zeros += 1;
                }
            }
            last = c;
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_the_longest_balanced_substring() {
        assert_eq!(
            Solution::find_the_longest_balanced_substring("01000111".to_string()),
            6
        );
        assert_eq!(
            Solution::find_the_longest_balanced_substring("00111".to_string()),
            4
        );
        assert_eq!(
            Solution::find_the_longest_balanced_substring("111".to_string()),
            0
        );
    }
}
