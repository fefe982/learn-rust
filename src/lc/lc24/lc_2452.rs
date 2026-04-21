// https://leetcode.cn/problems/words-within-two-edits-of-dictionary/
// 2452. Words Within Two Edits of Dictionary
pub struct Solution;
impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        let mut ans = Vec::new();
        for query in queries {
            for dict in &dictionary {
                let mut diff = 0;
                for (a, b) in query.chars().zip(dict.chars()) {
                    if a != b {
                        diff += 1;
                    }
                }
                if diff <= 2 {
                    ans.push(query);
                    break;
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_two_edit_words() {
        assert_eq!(
            Solution::two_edit_words(
                vec_str!["word", "note", "ants", "wood"],
                vec_str!["wood", "joke", "moat"]
            ),
            vec_str!["word", "note", "wood"]
        );
        assert_eq!(
            Solution::two_edit_words(vec_str!["yes"], vec_str!["not"]),
            Vec::<String>::new()
        );
    }
}
