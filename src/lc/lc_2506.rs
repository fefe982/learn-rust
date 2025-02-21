// https://leetcode.com/problems/count-pairs-of-similar-strings/
// 2506. Count Pairs of Similar Strings
pub struct Solution;
impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut sum = 0;
        let mut s = std::collections::HashMap::new();
        for w in words {
            let mut c = 0;
            for b in w.bytes() {
                c |= 1 << (b - b'a');
            }
            if let Some(&n) = s.get(&c) {
                sum += n;
                s.insert(c, n + 1);
            } else {
                s.insert(c, 1);
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_similar_pairs() {
        assert_eq!(
            Solution::similar_pairs(vec_str!["aba", "aabb", "abcd", "bac", "aabc"]),
            2
        );
        assert_eq!(Solution::similar_pairs(vec_str!["aabb", "ab", "ba"]), 3);
        assert_eq!(Solution::similar_pairs(vec_str!["nba", "cba", "dba"]), 0);
    }
}
