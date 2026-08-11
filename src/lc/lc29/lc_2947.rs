// https://leetcode.com/problems/count-beautiful-substrings-i/
// 2947. Count Beautiful Substrings I
pub struct Solution;
impl Solution {
    pub fn beautiful_substrings(s: String, k: i32) -> i32 {
        super::lc_2949::Solution::beautiful_substrings(s, k) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn beautiful_substrings() {
        assert_eq!(Solution::beautiful_substrings("baeyh".to_string(), 2), 2);
        assert_eq!(Solution::beautiful_substrings("abba".to_string(), 1), 3);
        assert_eq!(Solution::beautiful_substrings("bcdf".to_string(), 1), 0);
    }
}
