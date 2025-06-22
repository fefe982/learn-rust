// https://leetcode.com/problems/word-ladder-ii/
// 126. Word Ladder II
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
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        mut word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
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
            return Vec::new();
        }
        if begin_word_idx == usize::MAX {
            begin_word_idx = word_list.len();
            word_list.push(begin_word);
        }
        let mut edges = std::collections::BTreeMap::<usize, Vec<usize>>::new();

        let mut que = std::collections::vec_deque::VecDeque::new();
        que.push_back(end_word_idx);
        let mut levels = vec![-1; word_list.len()];
        levels[end_word_idx] = 0;
        while let Some(word) = que.pop_front() {
            let level = levels[word];
            for idx in 0..word_list.len() {
                if levels[idx] == -1 || levels[idx] == level + 1 {
                    if Self::cmp_word(word_list[word].as_bytes(), word_list[idx].as_bytes()) {
                        if levels[idx] == -1 {
                            que.push_back(idx);
                            levels[idx] = level + 1;
                        }
                        edges.entry(idx).or_default().push(word);
                    }
                }
            }
        }
        if levels[begin_word_idx] == -1 {
            return Vec::new();
        }

        let mut result = Vec::new();
        let mut que = std::collections::vec_deque::VecDeque::new();
        que.push_back(vec![begin_word_idx]);
        while let Some(p) = que.pop_front() {
            let word = p.last().unwrap();
            if let Some(nodes) = edges.get(word) {
                for &idx in nodes {
                    let mut p = p.clone();
                    p.push(idx);
                    que.push_back(p);
                }
            } else {
                result.push(p.iter().map(|&x| word_list[x].clone()).collect());
            }
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_str;
    use crate::vec_vec_str;
    #[test]
    fn find_ladders() {
        assert_eq!(
            Solution::find_ladders(
                String::from("hot"),
                String::from("dog"),
                vec_str!["hot", "dog"]
            ),
            Vec::<Vec<String>>::new()
        );
        assert_eq!(
            Solution::find_ladders(
                String::from("red"),
                String::from("tax"),
                vec_str!["ted", "tex", "red", "tax", "tad", "den", "rex", "pee"]
            ),
            vec_vec_str![
                ["red", "ted", "tex", "tax"],
                ["red", "ted", "tad", "tax"],
                ["red", "rex", "tex", "tax"]
            ]
        );
        assert_eq!(
            Solution::find_ladders(
                String::from("hit"),
                String::from("cog"),
                vec_str!["hot", "dot", "dog", "lot", "log", "cog"]
            ),
            vec_vec_str![
                ["hit", "hot", "dot", "dog", "cog"],
                ["hit", "hot", "lot", "log", "cog"]
            ]
        );
        assert_eq!(
            Solution::find_ladders(
                String::from("hit"),
                String::from("cog"),
                vec_str!["hot", "dot", "dog", "lot", "log"]
            ),
            Vec::<Vec<String>>::new()
        );
    }
}
