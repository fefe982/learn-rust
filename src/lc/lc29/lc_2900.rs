// https://leetcode.cn/problems/longest-unequal-adjacent-groups-subsequence-i/
// 2900. Longest Unequal Adjacent Groups Subsequence I
pub struct Solution;
impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut res = vec![words[0].clone()];
        let mut idx = groups[0];
        for i in 1..words.len() {
            if groups[i] != idx {
                res.push(words[i].clone());
                idx = groups[i];
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
    fn get_longest_subsequence() {
        assert_eq!(
            Solution::get_longest_subsequence(vec_str!["e", "a", "b"], vec![0, 0, 1]),
            vec_str!["e", "b"]
        );
        assert_eq!(
            Solution::get_longest_subsequence(vec_str!["a", "b", "c", "d"], vec![1, 0, 1, 1]),
            vec_str!["a", "b", "c"]
        );
    }
}
