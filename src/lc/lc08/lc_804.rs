// https://leetcode.com/problems/unique-morse-code-words/
// 804. Unique Morse Code Words
pub struct Solution;
impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let m = vec![
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--", "-.", "---",
            ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..",
        ];
        let mut set = std::collections::HashSet::new();
        for word in words {
            let mut morse = String::new();
            for c in word.chars() {
                morse.push_str(m[c as usize - 'a' as usize]);
            }
            set.insert(morse);
        }
        set.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_unique_morse_representations() {
        assert_eq!(
            Solution::unique_morse_representations(vec_str!["gin", "zen", "gig", "msg"]),
            2
        );
        assert_eq!(Solution::unique_morse_representations(vec_str!["a"]), 1);
    }
}
