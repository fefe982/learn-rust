// https://leetcode.com/problems/remove-adjacent-almost-equal-characters/
// 2957. Remove Adjacent Almost Equal Characters
pub struct Solution;
impl Solution {
    pub fn remove_almost_equal_characters(word: String) -> i32 {
        let mut l = 0;
        let mut last = b'.';
        let mut ans = 0;
        for &c in word.as_bytes() {
            if (c as i32 - last as i32).abs() <= 1 {
                l += 1;
            } else {
                ans += l / 2;
                l = 1;
            }
            last = c;
        }
        ans + l / 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn remove_almost_equal_characters() {
        assert_eq!(Solution::remove_almost_equal_characters("aaaaa".to_string()), 2);
        assert_eq!(Solution::remove_almost_equal_characters("abddez".to_string()), 2);
        assert_eq!(Solution::remove_almost_equal_characters("zyxyxyz".to_string()), 3);
    }
}
