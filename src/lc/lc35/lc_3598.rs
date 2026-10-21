// https://leetcode.com/problems/longest-common-prefix-between-adjacent-strings-after-removals/
// 3598. Longest Common Prefix Between Adjacent Strings After Removals
pub struct Solution;
impl Solution {
    pub fn longest_common_prefix(words: Vec<String>) -> Vec<i32> {
        if words.len() <= 2 {
            return std::iter::repeat(0).take(words.len()).collect();
        }
        fn lcp(a: &str, b: &str) -> i32 {
            let mut i = 0;
            while i < a.len() && i < b.len() && a.as_bytes()[i] == b.as_bytes()[i] {
                i += 1;
            }
            i as i32
        }
        let mut adj = vec![];
        let mut left = vec![0];
        let mut lm = 0;
        for i in 1..words.len() {
            let c = lcp(&words[i - 1], &words[i]);
            adj.push(c);
            lm = lm.max(c);
            left.push(lm);
        }
        let mut right = vec![0; words.len()];
        let mut rm = 0;
        for i in (0..words.len() - 1).rev() {
            rm = rm.max(adj[i]);
            right[i] = rm;
        }
        let mut ans = vec![0; words.len()];
        ans[0] = right[1];
        ans[words.len() - 1] = left[words.len() - 2];
        for i in 1..words.len() - 1 {
            let c = lcp(&words[i - 1], &words[i + 1]);
            ans[i] = c.max(left[i - 1]).max(right[i + 1]);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn longest_common_prefix() {
        assert_eq!(
            Solution::longest_common_prefix(vec_str!["jump", "run", "run", "jump", "run"]),
            vec![3, 0, 0, 3, 3]
        );
        assert_eq!(
            Solution::longest_common_prefix(vec_str!["dog", "racer", "car"]),
            vec![0, 0, 0]
        );
    }
}
