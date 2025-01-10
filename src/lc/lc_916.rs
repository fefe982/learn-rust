// https://leetcode.com/problems/word-subsets/
// 916. Word Subsets
pub struct Solution;
impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut cnt2 = [0; 26];
        for w in words2 {
            let mut cnt = [0; 26];
            for c in w.chars() {
                cnt[c as usize - 'a' as usize] += 1;
            }
            for i in 0..26 {
                cnt2[i] = cnt2[i].max(cnt[i]);
            }
        }
        words1
            .into_iter()
            .filter(|w| {
                let mut cnt = [0; 26];
                for c in w.chars() {
                    cnt[c as usize - 'a' as usize] += 1;
                }
                for i in 0..26 {
                    if cnt[i] < cnt2[i] {
                        return false;
                    }
                }
                true
            })
            .collect()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::word_subsets(
                vec_str!["amazon", "apple", "facebook", "google", "leetcode"],
                vec_str!["e", "o"]
            ),
            vec_str!["facebook", "google", "leetcode"]
        );
        assert_eq!(
            Solution::word_subsets(
                vec_str!["amazon", "apple", "facebook", "google", "leetcode"],
                vec_str!["l", "e"]
            ),
            vec_str!["apple", "google", "leetcode"]
        );
    }
}
