// https://leetcode.com/problems/count-palindromic-subsequences/
// 2484. Count Palindromic Subsequences
pub struct Solution;
impl Solution {
    pub fn count_palindromes(s: String) -> i32 {
        let s = s.as_bytes().iter().map(|&c| (c - b'0') as usize).collect::<Vec<_>>();
        if s.len() < 5 {
            return 0;
        }
        let n = s.len();
        let mut pre = vec![vec![vec![0; 10]; 10]; n - 3];
        let mut cnt = vec![0i64; 10];
        let m = 1_0000_0000_7;
        for i in 0..n - 3 {
            if i > 0 {
                for j in 0..10 {
                    for k in 0..10 {
                        pre[i][j][k] = pre[i - 1][j][k];
                        if k == s[i] {
                            pre[i][j][k] = (pre[i][j][k] + cnt[j]) % m;
                        }
                    }
                }
            }
            cnt[s[i]] += 1;
        }
        for i in 0..10 {
            cnt[i] = 0;
        }
        let mut post = vec![vec![0; 10]; 10];
        let mut ans = 0;
        for i in (3..n).rev() {
            let mut npost = vec![vec![0; 10]; 10];
            if i < n - 1 && i > 2 {
                for j in 0..10 {
                    for k in 0..10 {
                        npost[j][k] = post[j][k];
                        if k == s[i] {
                            npost[j][k] = (npost[j][k] + cnt[j]) % m;
                        }
                        ans = (ans + (pre[i - 2][j][k] * npost[j][k])) % m;
                    }
                }
                post = npost;
            }
            cnt[s[i]] += 1;
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_palindromes() {
        assert_eq!(Solution::count_palindromes("103301".to_string()), 2);
        assert_eq!(Solution::count_palindromes("0000000".to_string()), 21);
        assert_eq!(Solution::count_palindromes("9999900000".to_string()), 2);
    }
}
