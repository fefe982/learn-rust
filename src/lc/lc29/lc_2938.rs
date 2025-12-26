// https://leetcode.com/problems/separate-black-and-white-balls/
// 2938. Separate Black and White Balls
pub struct Solution;
impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        s.bytes()
            .fold((0i64, 0i64), |(total, black), c| {
                if c == b'1' {
                    (total, black + 1)
                } else {
                    (total + black, black)
                }
            })
            .0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_steps() {
        assert_eq!(Solution::minimum_steps("101".to_string()), 1);
        assert_eq!(Solution::minimum_steps("100".to_string()), 2);
        assert_eq!(Solution::minimum_steps("0111".to_string()), 0);
    }
}
