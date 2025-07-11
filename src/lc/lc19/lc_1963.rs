// https://leetcode.com/problems/minimum-number-of-swaps-to-make-the-string-balanced/
// 1963. Minimum Number of Swaps to Make the String Balanced
pub struct Solution;
impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut min = 0;
        let mut cnt = 0;
        for c in s.chars() {
            if c == '[' {
                cnt += 1;
            } else {
                cnt -= 1;
                min = min.min(cnt);
            }
        }
        (1 - min) / 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_swaps() {
        assert_eq!(Solution::min_swaps("][][".to_string()), 1);
        assert_eq!(Solution::min_swaps("]]][[[".to_string()), 2);
        assert_eq!(Solution::min_swaps("[]".to_string()), 0);
    }
}
