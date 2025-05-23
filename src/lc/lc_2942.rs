// https://leetcode.com/problems/find-words-containing-character/
// 2942. Find Words Containing All Characters
pub struct Solution;
impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut r = Vec::with_capacity(words.len());
        for (i, w) in words.into_iter().enumerate() {
            if w.find(x).is_some() {
                r.push(i as i32);
            }
        }
        r
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_words_containing() {
        assert_eq!(Solution::find_words_containing(vec_str!["leet", "code"], 'e'), [0, 1]);
        assert_eq!(
            Solution::find_words_containing(vec_str!["abc", "bcd", "aaaa", "cbc"], 'a'),
            vec![0, 2]
        );
        assert_eq!(
            Solution::find_words_containing(vec_str!["abc", "bcd", "aaaa", "cbc"], 'z'),
            vec![]
        );
    }
}
