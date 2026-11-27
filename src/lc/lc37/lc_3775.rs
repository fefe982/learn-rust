// https://leetcode.com/problems/reverse-words-with-same-vowel-count/
// 3775. Reverse Words With Same Vowel Count
pub struct Solution;
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let words = s.split_whitespace().collect::<Vec<_>>();
        let vowels = vec!['a', 'e', 'i', 'o', 'u'];
        let mut vowel_count = vec![0; words.len()];
        for (i, word) in words.iter().enumerate() {
            for c in word.chars() {
                if vowels.contains(&c) {
                    vowel_count[i] += 1;
                }
            }
        }
        let mut ss = String::with_capacity(s.len());
        ss.push_str(words[0]);
        for i in 1..words.len() {
            ss.push(' ');
            if vowel_count[i] == vowel_count[0] {
                for c in words[i].chars().rev() {
                    ss.push(c);
                }
            } else {
                ss.push_str(words[i]);
            }
        }
        ss
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reverse_words() {
        assert_eq!(
            Solution::reverse_words("cat and mice".to_string()),
            "cat dna mice".to_string()
        );
        assert_eq!(
            Solution::reverse_words("book is nice".to_string()),
            "book is ecin".to_string()
        );
        assert_eq!(
            Solution::reverse_words("banana healthy".to_string()),
            "banana healthy".to_string()
        );
    }
}
