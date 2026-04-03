// https://leetcode.com/problems/check-if-the-sentence-is-pangram/
// 1832. Check if the Sentence Is Pangram
pub struct Solution;
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut seen = [false; 26];
        for b in sentence.as_bytes() {
            seen[(b - b'a') as usize] = true;
        }
        seen.into_iter().all(|x| x)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_if_pangram() {
        assert_eq!(
            Solution::check_if_pangram("thequickbrownfoxjumpsoverthelazydog".to_string()),
            true
        );
        assert_eq!(Solution::check_if_pangram("leetcode".to_string()), false);
    }
}
