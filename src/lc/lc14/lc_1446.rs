// https://leetcode.com/problems/consecutive-characters/
// 1446. Consecutive Characters
pub struct Solution;
impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut max = 1;
        let mut cnt = 1;
        let mut ch = ' ';
        for c in s.chars() {
            if c == ch {
                cnt += 1;
                max = max.max(cnt);
            } else {
                ch = c;
                cnt = 1;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_power() {
        assert_eq!(Solution::max_power("leetcode".to_string()), 2);
        assert_eq!(Solution::max_power("abbcccddddeeeeedcba".to_string()), 5);
    }
}
