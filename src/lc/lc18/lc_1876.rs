// https://leetcode.com/problems/substrings-of-size-three-with-distinct-characters/
// 1876. Substrings of Size Three with Distinct Characters
pub struct Solution;
impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        let bytes = s.as_bytes();
        if bytes.len() < 3 {
            return 0;
        }
        let mut count = 0;
        for i in 0..bytes.len() - 2 {
            if bytes[i] != bytes[i + 1] && bytes[i] != bytes[i + 2] && bytes[i + 1] != bytes[i + 2] {
                count += 1;
            }
        }
        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_good_substrings() {
        assert_eq!(Solution::count_good_substrings("xyzzaz".to_string()), 1);
        assert_eq!(Solution::count_good_substrings("aababcabc".to_string()), 4);
    }
}
