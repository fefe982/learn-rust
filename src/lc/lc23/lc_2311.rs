// https://leetcode.com/problems/longest-binary-subsequence-less-than-or-equal-to-k
// 2311. Longest Binary Subsequence Less Than or Equal to K
pub struct Solution;
impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let mut ans = 0;
        let mut p = 1;
        let mut sum = 0;
        for i in s.bytes().rev() {
            if i == b'0' {
                ans += 1;
                if p <= k {
                    p *= 2;
                }
            } else if p <= k - sum {
                ans += 1;
                sum += p;
                p *= 2;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_subsequence() {
        assert_eq!(Solution::longest_subsequence("1001010".to_string(), 5), 5);
        assert_eq!(Solution::longest_subsequence("00101001".to_string(), 1), 6);
    }
}
