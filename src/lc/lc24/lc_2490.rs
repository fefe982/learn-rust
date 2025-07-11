// https://leetcode.com/problems/circular-sentence/
// 2490. Circular Sentence
pub struct Solution;
impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let words = sentence.split_ascii_whitespace().collect::<Vec<&str>>();
        if words.first().unwrap().as_bytes().first() != words.last().unwrap().as_bytes().last() {
            return false;
        }
        for i in 1..words.len() {
            if words[i - 1].as_bytes().last() != words[i].as_bytes().first() {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_circular_sentence() {
        assert_eq!(
            Solution::is_circular_sentence(String::from("leetcode exercises sound delightful")),
            true
        );
        assert_eq!(
            Solution::is_circular_sentence(String::from("eetcode")),
            true
        );
        assert_eq!(
            Solution::is_circular_sentence(String::from("Leetcode is cool")),
            false
        );
    }
}
