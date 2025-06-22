// https://leetcode.com/problems/length-of-last-word/
// 58. Length of Last Word
pub struct Solution;
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().map_or(0, |x| x.len() as i32)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length_of_last_word() {
        assert_eq!(Solution::length_of_last_word(String::from("Hello World")), 5);
        assert_eq!(
            Solution::length_of_last_word(String::from("   fly me   to   the moon  ")),
            4
        );
        assert_eq!(Solution::length_of_last_word(String::from("luffy is still joyboy")), 6);
    }
}
