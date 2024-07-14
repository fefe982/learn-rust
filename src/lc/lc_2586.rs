// https://leetcode.com/problems/count-the-number-of-vowel-strings-in-range/
// 2586. Count the Number of Vowel Strings in Range
pub struct Solution;
impl Solution {
    fn vowel(c: char) -> bool {
        c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
    }
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        words[left as usize..=right as usize]
            .iter()
            .filter(|&w| {
                let mut cc = w.chars();
                let c = cc.next().unwrap();
                if !Self::vowel(c) {
                    false
                } else if let Some(cl) = cc.last() {
                    Self::vowel(cl)
                } else {
                    true
                }
            })
            .count() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_vowel_strings() {
        assert_eq!(Solution::vowel_strings(vec_str!["are", "amy", "u"], 0, 2), 2);
        assert_eq!(
            Solution::vowel_strings(vec_str!["hey", "aeo", "mu", "ooo", "artro"], 1, 4),
            3
        );
    }
}
