// https://leetcode.com/problems/sorting-the-sentence/
// 1859. Sorting the Sentence
pub struct Solution;
impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut words = s.split_whitespace().collect::<Vec<_>>();
        words.sort_by_key(|w| w.chars().last().unwrap());
        words
            .into_iter()
            .map(|w| &w[..w.len() - 1])
            .collect::<Vec<_>>()
            .join(" ")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sort_sentence() {
        assert_eq!(
            Solution::sort_sentence("is2 sentence4 This1 a3".to_string()),
            "This is a sentence".to_string()
        );
        assert_eq!(
            Solution::sort_sentence("Myself2 Me1 I4 and3".to_string()),
            "Me Myself and I".to_string()
        );
    }
}
