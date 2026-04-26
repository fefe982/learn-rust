// https://leetcode.com/problems/minimum-time-to-type-word-using-special-typewriter/
// 1974. Minimum Time to Type Word Using Special Typewriter
pub struct Solution;
impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let mut time = 0;
        let mut prev = 'a';
        for c in word.chars() {
            let dist = (c as i32 - prev as i32).abs();
            time += dist.min(26 - dist) + 1;
            prev = c;
        }
        time
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_time_to_type() {
        assert_eq!(Solution::min_time_to_type("abc".to_string()), 5);
        assert_eq!(Solution::min_time_to_type("bza".to_string()), 7);
        assert_eq!(Solution::min_time_to_type("zjpc".to_string()), 34);
    }
}
