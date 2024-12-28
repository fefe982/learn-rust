// https://leetcode.com/problems/longest-palindromic-substring/
// 5. Longest Palindromic Substring
pub struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.chars().collect::<Vec<char>>();
        let mut max_cnt = 0;
        let mut max_i = 0;
        for i in 0..s.len() {
            let mut p = i.wrapping_sub(1);
            let mut q = i + 1;
            let mut cnt = 1;
            while p < s.len() && q < s.len() && s[p] == s[q] {
                p = p.wrapping_sub(1);
                q += 1;
                cnt += 2;
            }
            if cnt > max_cnt {
                max_cnt = cnt;
                max_i = i;
            }
            p = i;
            q = i + 1;
            cnt = 0;
            while p < s.len() && q < s.len() && s[p] == s[q] {
                p = p.wrapping_sub(1);
                q += 1;
                cnt += 2;
            }
            if cnt > max_cnt {
                max_cnt = cnt;
                max_i = i;
            }
        }
        let left = max_i - (max_cnt - 1) / 2;
        let right = left + max_cnt;
        s[left..right].iter().collect::<String>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_palindrome() {
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab".to_string());
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb".to_string());
    }
}
