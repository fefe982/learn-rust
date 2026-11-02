// https://leetcode.com/problems/minimum-operations-to-transform-string/
// 3675. Minimum Operations to Transform String
pub struct Solution;
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut min = 'z';
        let mut has_non_a = false;
        for c in s.chars() {
            if c != 'a' {
                has_non_a = true;
                min = min.min(c);
            }
        }
        if !has_non_a {
            return 0;
        }
        'z' as i32 - min as i32 + 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations("ae".to_string()), 22);
        assert_eq!(Solution::min_operations("yz".to_string()), 2);
        assert_eq!(Solution::min_operations("a".to_string()), 0);
    }
}
