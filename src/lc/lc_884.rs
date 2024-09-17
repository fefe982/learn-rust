// https://leetcode.com/problems/uncommon-words-from-two-sentences/
// 884. Uncommon Words from Two Sentences
pub struct Solution;
impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut m1 = std::collections::HashMap::<&str, i32>::new();
        let mut m2 = std::collections::HashMap::<&str, i32>::new();
        for s in s1.split(' ') {
            *m1.entry(s).or_default() += 1;
        }
        for s in s2.split(' ') {
            *m2.entry(s).or_default() += 1;
        }
        let mut ret = vec![];
        for (&k, &v) in &m1 {
            if v == 1 && !m2.contains_key(k) {
                ret.push(k.to_string());
            }
        }
        for (k, v) in m2 {
            if v == 1 && !m1.contains_key(k) {
                ret.push(k.to_string());
            }
        }
        ret
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_uncommon_from_sentences() {
        assert_eq!(
            Solution::uncommon_from_sentences("this apple is sweet".to_string(), "this apple is sour".to_string()),
            vec!["sweet", "sour"]
        );
        assert_eq!(
            Solution::uncommon_from_sentences("apple apple".to_string(), "banana".to_string()),
            vec!["banana"]
        );
    }
}
