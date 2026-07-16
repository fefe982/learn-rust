// https://leetcode.com/problems/decremental-string-concatenation/
// 2746. Decremental String Concatenation
pub struct Solution;
impl Solution {
    fn dfs(i: usize, words: &Vec<String>, s: u8, e: u8, dp: &mut Vec<[[i32; 26]; 26]>) -> i32 {
        if i == words.len() {
            return 0;
        }
        if dp[i][(s - b'a') as usize][(e - b'a') as usize] != -1 {
            return dp[i][(s - b'a') as usize][(e - b'a') as usize];
        }
        let w = words[i].as_bytes();
        let mut r1 = w.len() as i32 + Self::dfs(i + 1, words, s, w[w.len() - 1], dp);
        if e == w[0] {
            r1 -= 1;
        }
        let mut r2 = w.len() as i32 + Self::dfs(i + 1, words, w[0], e, dp);
        if s == w[w.len() - 1] {
            r2 -= 1;
        }
        let r = r1.min(r2);
        dp[i][(s - b'a') as usize][(e - b'a') as usize] = r;
        r
    }
    pub fn minimize_concatenated_length(words: Vec<String>) -> i32 {
        let nw = words.len();
        let mut dp = vec![[[-1; 26]; 26]; nw];
        let w0 = words[0].as_bytes();
        w0.len() as i32 + Self::dfs(1, &words, w0[0], w0[w0.len() - 1], &mut dp)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn minimize_concatenated_length() {
        assert_eq!(Solution::minimize_concatenated_length(vec_str!["aa", "ab", "bc"]), 4);
        assert_eq!(Solution::minimize_concatenated_length(vec_str!["ab", "b"]), 2);
        assert_eq!(Solution::minimize_concatenated_length(vec_str!["aaa", "c", "aba"]), 6);
    }
}
