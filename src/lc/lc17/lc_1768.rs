// https://leetcode.com/problems/merge-strings-alternately/
// 1768. Merge Strings Alternately
pub struct Solution;
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let l = std::cmp::min(word1.len(), word2.len());
        let mut res = Vec::with_capacity(word1.len() + word2.len());
        for idx in 0..l {
            res.push(word1[idx]);
            res.push(word2[idx]);
        }
        for idx in l..word1.len() {
            res.push(word1[idx]);
        }
        for idx in l..word2.len() {
            res.push(word2[idx]);
        }
        String::from_utf8(res).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn merge_alternately() {
        assert_eq!(
            Solution::merge_alternately(String::from("abc"), String::from("pqr")),
            String::from("apbqcr")
        );
        assert_eq!(
            Solution::merge_alternately(String::from("ab"), String::from("pqrs")),
            String::from("apbqrs")
        );
        assert_eq!(
            Solution::merge_alternately(String::from("abcd"), String::from("pq")),
            String::from("apbqcd")
        );
    }
}
