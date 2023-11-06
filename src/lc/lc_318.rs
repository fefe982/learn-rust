// https://leetcode.com/problems/maximum-product-of-word-lengths/
// 318. Maximum Product of Word Lengths
pub struct Solution;
impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let words = words
            .into_iter()
            .map(|s| {
                let s = s.as_bytes();
                let l = s.len();
                let mut n = 0;
                for c in s {
                    n |= 1 << (c - 'a' as u8);
                }
                (n, l as i32)
            })
            .collect::<Vec<_>>();
        let mut max = 0;
        for i in 0..words.len() - 1 {
            for j in 1..words.len() {
                if (words[i].0 & words[j].0) == 0 {
                    max = max.max(words[i].1 * words[j].1);
                }
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_product() {
        assert_eq!(
            Solution::max_product(vec_str!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"]),
            16
        );
        assert_eq!(
            Solution::max_product(vec_str!["a", "ab", "abc", "d", "cd", "bcd", "abcd"]),
            4
        );
        assert_eq!(Solution::max_product(vec_str!["a", "aa", "aaa", "aaaa"]), 0);
    }
}
