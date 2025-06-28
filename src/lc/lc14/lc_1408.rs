// https://leetcode.com/problems/string-matching-in-an-array/
// 1408. String Matching in an Array
pub struct Solution;
impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut words = words;
        words.sort_by(|a, b| a.len().cmp(&b.len()));
        let mut res = vec![];
        for iw in 0..words.len() - 1 {
            let w = words[iw].as_bytes();
            let mut lps = vec![0; w.len()];
            let mut i = 0;
            for j in 1..w.len() {
                while i > 0 && w[i] != w[j] {
                    i = lps[i - 1];
                }
                if w[i] == w[j] {
                    i += 1;
                    lps[j] = i;
                }
            }
            i = 0;
            for &c in words[iw + 1..]
                .iter()
                .map(|s| s.as_bytes().iter().chain([b'#'].iter()))
                .flatten()
            {
                while i > 0 && w[i] != c {
                    i = lps[i - 1];
                }
                if w[i] == c {
                    i += 1;
                }
                if i == w.len() {
                    res.push(words[iw].clone());
                    break;
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn string_matching() {
        assert_sort_eq!(
            Solution::string_matching(vec_str!["mass", "as", "hero", "superhero"]),
            ["as", "hero"]
        );
        assert_eq!(
            Solution::string_matching(vec_str!["leetcode", "et", "code"]),
            ["et", "code"]
        );
        assert_eq!(
            Solution::string_matching(vec_str!["blue", "green", "bu"]),
            Vec::<String>::new()
        );
    }
}
