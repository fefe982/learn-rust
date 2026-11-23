// https://leetcode.com/problems/maximum-substrings-with-distinct-start/
// 3760. Maximum Substrings With Distinct Start
pub struct Solution;
impl Solution {
    pub fn max_distinct(s: String) -> i32 {
        let mut mask = 0;
        let mut cnt = 0;
        for c in s.chars() {
            let bit = 1 << (c as u8 - 'a' as u8);
            if mask & bit == 0 {
                mask |= bit;
                cnt += 1;
            }
            if cnt == 26 {
                break;
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_distinct() {
        assert_eq!(Solution::max_distinct("abab".to_string()), 2);
        assert_eq!(Solution::max_distinct("abcd".to_string()), 4);
        assert_eq!(Solution::max_distinct("aaaa".to_string()), 1);
    }
}
