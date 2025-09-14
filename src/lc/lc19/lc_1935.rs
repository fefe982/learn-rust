// https://leetcode.com/problems/maximum-number-of-words-you-can-type/
// 1935. Maximum Number of Words You Can Type
pub struct Solution;
impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut v = [0; 26];
        for c in broken_letters.chars() {
            v[(c as u8 - b'a') as usize] += 1;
        }
        let mut ans = 0;
        for w in text.split_ascii_whitespace() {
            let mut flag = true;
            for c in w.chars() {
                if v[(c as u8 - b'a') as usize] > 0 {
                    flag = false;
                    break;
                }
            }
            if flag {
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
    fn can_be_typed_words() {
        assert_eq!(
            Solution::can_be_typed_words("hello world".to_string(), "ad".to_string()),
            1
        );
        assert_eq!(
            Solution::can_be_typed_words("leet code".to_string(), "lt".to_string()),
            1
        );
        assert_eq!(
            Solution::can_be_typed_words("leet code".to_string(), "e".to_string()),
            0
        );
    }
}
