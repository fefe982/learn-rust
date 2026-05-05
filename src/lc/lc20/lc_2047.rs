// https://leetcode.com/problems/number-of-valid-words-in-a-sentence/
// 2047. Number of Valid Words in a Sentence
pub struct Solution;
impl Solution {
    pub fn count_valid_words(sentence: String) -> i32 {
        let mut ans = 0;
        for word in sentence.split_whitespace() {
            let mut hyphen_idx = usize::MAX;
            let mut valid = true;
            for (i, c) in word.chars().enumerate() {
                if c.is_ascii_digit() {
                    valid = false;
                    break;
                }
                if c == '-' {
                    if hyphen_idx != usize::MAX || i == 0 || i == word.len() - 1 {
                        valid = false;
                        break;
                    }
                    hyphen_idx = i;
                } else if hyphen_idx != usize::MAX && hyphen_idx + 1 == i && !c.is_ascii_lowercase() {
                    valid = false;
                    break;
                } else if !c.is_ascii_lowercase() {
                    if i != word.len() - 1 {
                        valid = false;
                        break;
                    }
                    if c != '!' && c != '.' && c != ',' {
                        valid = false;
                        break;
                    }
                }
            }
            if valid {
                ans += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_valid_words() {
        assert_eq!(Solution::count_valid_words("cat and  dog".to_string()), 3);
        assert_eq!(Solution::count_valid_words("!this  1-s b8d!".to_string()), 0);
        assert_eq!(
            Solution::count_valid_words("alice and  bob are playing stone-game10".to_string()),
            5
        );
    }
}
