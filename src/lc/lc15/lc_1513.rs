// https://leetcode.com/problems/number-of-substrings-with-only-1s/
// 1513. Number of Substrings with Only 1s
pub struct Solution;
impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let mut ans = 0;
        let mut cnt = 0;
        for c in s.chars() {
            if c == '1' {
                cnt += 1;
                ans += cnt as i64;
            } else {
                cnt = 0;
            }
        }
        (ans % 1_0000_0000_7) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_sub() {
        assert_eq!(Solution::num_sub("0110111".to_string()), 9);
        assert_eq!(Solution::num_sub("101".to_string()), 2);
        assert_eq!(Solution::num_sub("111111".to_string()), 21);
    }
}
