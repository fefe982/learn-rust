// https://leetcode.com/problems/maximum-score-after-splitting-a-string/
// 1422. Maximum Score After Splitting a String
pub struct Solution;
impl Solution {
    pub fn max_score(s: String) -> i32 {
        let len = s.len();
        let s = s.as_bytes();
        let mut max = 0;
        let mut cur = 0;
        let mut score = if s[0] == b'0' { 1 } else { 0 } + if s[len - 1] == b'1' { 1 } else { 0 };
        for i in 1..len - 1 {
            if s[i] == b'0' {
                cur += 1;
                score += 1;
            } else {
                cur -= 1;
            }
            max = max.max(cur);
        }
        max + score - cur
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_score() {
        assert_eq!(Solution::max_score("011101".to_string()), 5);
        assert_eq!(Solution::max_score("00111".to_string()), 5);
        assert_eq!(Solution::max_score("1111".to_string()), 3);
    }
}
