// https://leetcode.com/problems/number-of-different-integers-in-a-string/
// 1805. Number of Different Integers in a String
pub struct Solution;
impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        use std::collections::HashSet;
        let mut set: HashSet<String> = HashSet::new();
        let bytes = word.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            if !bytes[i].is_ascii_digit() {
                i += 1;
                continue;
            }

            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }

            let s = &word[start..i];
            let normalized = s.trim_start_matches('0');
            set.insert(normalized.to_string());
        }

        set.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_different_integers() {
        assert_eq!(Solution::num_different_integers("a123bc34d8ef34".to_string()), 3);
        assert_eq!(Solution::num_different_integers("leet1234code234".to_string()), 2);
        assert_eq!(Solution::num_different_integers("a1b01c001".to_string()), 1);
    }
}
