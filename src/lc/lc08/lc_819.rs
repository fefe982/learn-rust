// https://leetcode.com/problems/most-common-word/
// 819. Most Common Word
pub struct Solution;
impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        use std::collections::HashMap;
        use std::collections::HashSet;
        let mut word_count = HashMap::new();
        let banned = banned.into_iter().collect::<HashSet<_>>();
        let mut max = 0;
        let mut res = String::new();
        paragraph
            .to_lowercase()
            .split(&['!', '?', '\'', ',', ';', '.', ' '][..])
            .for_each(|w| {
                if banned.contains(w) || w.is_empty() {
                    return;
                }
                let c = *word_count.entry(w).and_modify(|count| *count += 1).or_insert(1);
                if c > max {
                    max = c;
                    res = w.to_string();
                }
            });
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn most_common_word() {
        assert_eq!(
            Solution::most_common_word("Bob. hIt, baLl".to_string(), vec_str!["bob", "hit"]),
            "ball"
        );
        assert_eq!(
            Solution::most_common_word(
                "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
                vec_str!["hit"]
            ),
            "ball"
        );
        assert_eq!(Solution::most_common_word("a.".to_string(), vec_str![]), "a");
    }
}
