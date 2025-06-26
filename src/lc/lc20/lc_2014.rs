// https://leetcode.com/problems/longest-subsequence-repeated-k-times/
// 2014. Longest Subsequence Repeated k Times
pub struct Solution;
impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        let s = s.as_bytes();
        let mut q = std::collections::VecDeque::new();
        q.push_back(vec![]);
        while let Some(mut c) = q.pop_front() {
            for i in 0..26 {
                let nc = b'a' + i;
                c.push(nc);
                let mut j = 0;
                let mut repeat = 0;
                for is in 0..s.len() {
                    if c[j] == s[is] {
                        j += 1;
                    }
                    if j == c.len() {
                        repeat += 1;
                        if repeat == k {
                            q.push_back(c.clone());
                            break;
                        }
                        j = 0;
                    }
                }
                c.pop();
            }
            if q.is_empty() {
                return String::from_utf8(c).unwrap();
            }
        }
        "".to_owned()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_subsequence_repeated_k() {
        assert_eq!(
            Solution::longest_subsequence_repeated_k("letsleetcode".to_string(), 2),
            "let"
        );
        assert_eq!(Solution::longest_subsequence_repeated_k("bb".to_string(), 2), "b");
        assert_eq!(Solution::longest_subsequence_repeated_k("ab".to_string(), 2), "");
    }
}
