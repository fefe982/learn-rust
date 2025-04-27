// https://leetcode.com/problems/keyboard-row/
// 500. Keyboard Row
pub struct Solution;
impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let group = [
            2, 3, 3, 2, 1, 2, 2, 2, 1, 2, 2, 2, 3, 3, 1, 1, 1, 1, 2, 1, 1, 3, 1, 3, 1, 3,
        ];
        let mut res = Vec::new();
        for word in words {
            let mut row = 0;
            for c in word.chars() {
                let c = c.to_ascii_lowercase() as usize - 'a' as usize;
                if row == 0 {
                    row = group[c];
                } else if row != group[c] {
                    row = 0;
                    break;
                }
            }
            if row != 0 {
                res.push(word);
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_words() {
        assert_eq!(
            Solution::find_words(vec_str!["Hello", "Alaska", "Dad", "Peace"]),
            ["Alaska", "Dad"]
        );
        assert_eq!(Solution::find_words(vec_str!["omk"]), Vec::<String>::new());
        assert_eq!(Solution::find_words(vec_str!["adsdf", "sfd"]), ["adsdf", "sfd"]);
    }
}
