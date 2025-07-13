// https://leetcode.com/problems/palindrome-partitioning-iii/
// 1278. Palindrome Partitioning III
pub struct Solution;
impl Solution {
    fn cnt_palindrome(s: &[u8]) -> i32 {
        let mut cnt = 0;
        for i in 0..s.len() / 2 {
            if s[i] != s[s.len() - i - 1] {
                cnt += 1;
            }
        }
        cnt
    }
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        let k = k as usize;
        let s = s.as_bytes();
        let mut cnt = vec![vec![vec![i32::MAX; s.len()]; s.len()]; k as usize];
        for i in 0..s.len() {
            for j in i..s.len() {
                cnt[0][i][j] = Solution::cnt_palindrome(&s[i..=j]);
            }
        }
        for ik in 1..k as usize {
            for i in 0..s.len() {
                for j in i + ik..s.len() {
                    for l in i..j - ik + 1 {
                        cnt[ik][i][j] = std::cmp::min(cnt[ik][i][j], cnt[0][i][l] + cnt[ik - 1][l + 1][j]);
                    }
                }
            }
        }
        cnt[k as usize - 1][0][s.len() - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_palindrome_partition() {
        assert_eq!(Solution::palindrome_partition("abc".to_string(), 2), 1);
        assert_eq!(Solution::palindrome_partition("aabbc".to_string(), 3), 0);
        assert_eq!(Solution::palindrome_partition("leetcode".to_string(), 8), 0);
    }
}
