// https://leetcode.com/problems/rearrange-words-in-a-sentence/
// 1451. Rearrange Words in a Sentence
pub struct Solution;
impl Solution {
    pub fn arrange_words(text: String) -> String {
        let mut words: Vec<&str> = text.split_whitespace().collect();
        words.sort_by_key(|w| w.len());
        let mut res = words.join(" ");
        if let Some(c) = res.get_mut(0..1) {
            c.make_ascii_uppercase();
        }
        if let Some(c) = res.get_mut(1..) {
            c.make_ascii_lowercase();
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arrange_words() {
        assert_eq!(
            Solution::arrange_words("Leetcode is cool".to_string()),
            "Is cool leetcode"
        );
        assert_eq!(
            Solution::arrange_words("Keep calm and code on".to_string()),
            "On and keep calm code"
        );
        assert_eq!(
            Solution::arrange_words("To be or not to be".to_string()),
            "To be or to be not"
        );
    }
}
