// https://leetcode.com/problems/number-of-matching-subsequences/
// 792. Number of Matching Subsequences
pub struct Solution;
impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut v = vec![vec![]; 26];
        for i in 0..words.len() {
            v[words[i].as_bytes()[0] as usize - 'a' as usize].push((i, 0));
        }
        let mut ans = 0;
        for c in s.as_bytes() {
            let i = *c as usize - 'a' as usize;
            let tmp = v[i].drain(..).collect::<Vec<_>>();
            for (wi, j) in tmp {
                if j + 1 == words[wi].len() {
                    ans += 1;
                } else {
                    v[words[wi].as_bytes()[j + 1] as usize - 'a' as usize].push((wi, j + 1));
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn num_matching_subseq() {
        assert_eq!(
            Solution::num_matching_subseq("abcde".to_string(), vec_str!["a", "bb", "acd", "ace"]),
            3
        );
        assert_eq!(
            Solution::num_matching_subseq(
                "dsahjpjauf".to_string(),
                vec_str!["ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"]
            ),
            2
        )
    }
}
