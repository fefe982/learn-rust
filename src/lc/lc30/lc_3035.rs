// https://leetcode.com/problems/maximum-palindromes-after-operations/
// 3035. Maximum Palindromes After Operations
pub struct Solution;
impl Solution {
    pub fn max_palindromes_after_operations(words: Vec<String>) -> i32 {
        let mut cnt = [0; 26];
        let mut len = Vec::with_capacity(words.len());
        for w in words {
            let mut c = 0;
            for b in w.bytes() {
                cnt[(b - b'a') as usize] += 1;
                c += 1;
            }
            len.push(c);
        }
        let mut e = 0;
        let mut o = 0;
        for i in 0..26 {
            e += cnt[i] / 2;
            o += cnt[i] % 2;
        }
        len.sort_unstable();
        let mut ans = 0;
        for l in len {
            if l % 2 == 1 {
                if o > 0 {
                    o -= 1;
                } else {
                    e -= 1;
                    o += 1;
                }
            }
            if e >= l / 2 {
                e -= l / 2;
                ans += 1;
            } else {
                break;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_palindromes_after_operations() {
        assert_eq!(
            Solution::max_palindromes_after_operations(vec_str!["abbb", "ba", "aa"]),
            3
        );
        assert_eq!(Solution::max_palindromes_after_operations(vec_str!["abc", "ab"]), 2);
        assert_eq!(Solution::max_palindromes_after_operations(vec_str!["cd", "ef", "a"]), 1);
    }
}
