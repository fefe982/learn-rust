// https://leetcode.com/problems/word-ladder/
// 127. Word Ladder
pub struct Solution;
impl Solution {
    fn cmp_word(w1: &[u8], w2: &[u8]) -> bool {
        let mut cnt = 0;
        for (&c1, &c2) in w1.iter().zip(w2.iter()) {
            if c1 != c2 {
                cnt += 1;
            }
            if cnt >= 2 {
                return false;
            }
        }
        cnt == 1
    }
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut word_list = word_list;
        let mut begin_word_idx = usize::MAX;
        let mut end_word_idx = usize::MAX;
        for (idx, w) in word_list.iter().enumerate() {
            if *w == begin_word {
                begin_word_idx = idx;
            }
            if *w == end_word {
                end_word_idx = idx;
            }
        }
        if end_word_idx == usize::MAX {
            return 0;
        }
        if begin_word_idx == usize::MAX {
            begin_word_idx = word_list.len();
            word_list.push(begin_word);
        }
        let mut que = std::collections::vec_deque::VecDeque::new();
        que.push_back(begin_word_idx);
        let mut levels = vec![0; word_list.len()];
        levels[begin_word_idx] = 1;
        while let Some(word) = que.pop_front() {
            let level = levels[word];
            for idx in 0..word_list.len() {
                if levels[idx] == 0 {
                    if Self::cmp_word(word_list[word].as_bytes(), word_list[idx].as_bytes()) {
                        if idx == end_word_idx {
                            return level + 1;
                        }
                        que.push_back(idx);
                        levels[idx] = level + 1;
                    }
                }
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_str;
    #[test]
    fn ladder_length() {
        assert_eq!(
            Solution::ladder_length(
                String::from("hit"),
                String::from("cog"),
                vec_str!["hot", "dot", "dog", "lot", "log", "cog"]
            ),
            5
        );
        assert_eq!(
            Solution::ladder_length(
                String::from("hit"),
                String::from("cog"),
                vec_str!["hot", "dot", "dog", "lot", "log"]
            ),
            0
        );
    }
}
