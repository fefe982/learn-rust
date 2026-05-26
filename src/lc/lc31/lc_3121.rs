// https://leetcode.com/problems/count-the-number-of-special-characters-ii/description/
// 3121. Count the Number of Special Characters II
pub struct Solution;
impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut count = [0; 26];
        for c in word.chars() {
            if c.is_ascii_lowercase() {
                let idx = (c as u8 - b'a') as usize;
                if count[idx] & 2 > 0 {
                    count[idx] |= 4;
                } else {
                    count[idx] |= 1;
                }
            } else if c.is_ascii_uppercase() {
                count[(c as u8 - b'A') as usize] |= 2;
            }
        }
        count.into_iter().filter(|&c| c == 3).count() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_special_chars() {
        assert_eq!(Solution::number_of_special_chars("aaAbcBC".to_string()), 3);
        assert_eq!(Solution::number_of_special_chars("abc".to_string()), 0);
        assert_eq!(Solution::number_of_special_chars("abBCab".to_string()), 0);
    }
}
