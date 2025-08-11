// https://leetcode.com/problems/split-strings-by-separator/
// 2788. Split Strings by Separator
pub struct Solution;
impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        words
            .into_iter()
            .map(|x| {
                x.split(separator)
                    .filter_map(|x| if x != "" { Some(x.to_string()) } else { None })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_split_words_by_separator() {
        assert_eq!(
            Solution::split_words_by_separator(vec_str!["one.two.three", "four.five", "six"], '.'),
            vec_str!["one", "two", "three", "four", "five", "six"]
        );
        assert_eq!(
            Solution::split_words_by_separator(vec_str!["$easy$", "$problem$"], '$'),
            vec_str!["easy", "problem"]
        );
        assert_eq!(
            Solution::split_words_by_separator(vec_str!["|||"], '|'),
            Vec::<String>::new()
        );
    }
}
