// https://leetcode.com/problems/longest-palindrome-after-substring-concatenation-ii/
// 3504. Longest Palindrome After Substring Concatenation II
pub struct Solution;
trait HasLength {
    fn len(&self) -> usize;
}
impl HasLength for [u8] {
    fn len(&self) -> usize {
        self.len()
    }
}
struct RevSlice<'a>(&'a [u8]);
impl<'a> HasLength for RevSlice<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
impl<'a> std::ops::Index<usize> for RevSlice<'a> {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[self.0.len() - 1 - index]
    }
}
impl Solution {
    fn max_length<T>(s: &T, t: &T) -> i32
    where
        T: ?Sized + HasLength + std::ops::Index<usize>,
        <T as std::ops::Index<usize>>::Output: PartialEq,
    {
        let ls = s.len();
        let lt = t.len();
        let mut dp = vec![vec![0; lt + 1]; ls + 1];
        let mut ans = 0;
        for i in 0..ls {
            for j in 0..lt {
                if s[i] == t[j] {
                    dp[i + 1][j] = dp[i][j + 1] + 1;
                    dp[i + 1][0] = dp[i + 1][0].max(dp[i + 1][j]);
                }
            }
            ans = ans.max(dp[i + 1][0]);
        }
        ans *= 2;
        for i in 0..(2 * ls - 1) {
            let mut l = i / 2;
            let mut r = (i + 1) / 2;
            if s[l] != s[r] {
                continue;
            }
            while l > 0 && r < ls - 1 && s[l - 1] == s[r + 1] {
                l -= 1;
                r += 1;
            }
            ans = ans.max((r - l + 1) as i32 + dp[l][0] * 2);
        }
        ans
    }
    pub fn longest_palindrome(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        Self::max_length(s, t).max(Self::max_length(&RevSlice(t), &RevSlice(s)))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_palindrome() {
        assert_eq!(Solution::longest_palindrome("a".to_string(), "a".to_string()), 2);
        assert_eq!(Solution::longest_palindrome("abc".to_string(), "def".to_string()), 1);
        assert_eq!(Solution::longest_palindrome("b".to_string(), "aaaa".to_string()), 4);
        assert_eq!(
            Solution::longest_palindrome("abcde".to_string(), "ecdba".to_string()),
            5
        );
    }
}
