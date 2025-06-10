// https://leetcode.com/problems/longest-word-in-dictionary/
// 720. Longest Word in Dictionary
pub struct Solution;
impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        let mut v = vec![];
        let mut words = words;
        let mut max = usize::MAX;
        words.sort();
        'w: for (i, word) in words.iter().enumerate() {
            let wl = word.len();
            while wl <= v.len() {
                v.pop();
            }
            if v.len() + 1 < wl {
                continue;
            }
            for (wc, vc) in word.chars().zip(v.iter()) {
                if wc != *vc {
                    continue 'w;
                }
            }
            v.push(word.chars().last().unwrap());
            if max == usize::MAX || wl > words[max].len() {
                max = i;
            }
        }
        if max == usize::MAX {
            return "".to_string();
        } else {
            return words[max].clone();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn longest_word() {
        assert_eq!(
            Solution::longest_word(vec_str![
                "rac", "rs", "ra", "on", "r", "otif", "o", "onpdu", "rsf", "rs", "ot", "oti", "racy", "onpd"
            ]),
            "otif"
        );
        assert_eq!(
            Solution::longest_word(vec_str!["w", "wo", "wor", "worl", "world"]),
            "world"
        );
        assert_eq!(
            Solution::longest_word(vec_str!["a", "banana", "app", "appl", "ap", "apply", "apple"]),
            "apple"
        );
    }
}
