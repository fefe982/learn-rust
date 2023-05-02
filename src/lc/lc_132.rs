// https://leetcode.com/problems/palindrome-partitioning-ii/
// 132. Palindrome Partitioning II
pub struct Solution;
impl Solution {
    fn palindrome(s: &[u8], start: usize, end: usize) -> bool {
        for idx in 0..(end - start) / 2 {
            if s[start + idx] != s[end - idx - 1] {
                return false;
            }
        }
        true
    }
    fn min_cut_slice(s: &[u8], start: usize, cuts: &mut Vec<i32>) -> i32 {
        if cuts[start] != 0 {
            return cuts[start];
        }
        let cut = if start + 1 == s.len() {
            1
        } else if start + 2 == s.len() {
            if s[start] == s[start + 1] {
                1
            } else {
                2
            }
        } else if Self::palindrome(s, start, s.len()) {
            1
        } else {
            let mut mcut = Self::min_cut_slice(s, start + 1, cuts) + 1;
            for idx in start + 2..s.len() {
                if Self::palindrome(s, start, idx) {
                    mcut = std::cmp::min(mcut, Self::min_cut_slice(s, idx, cuts) + 1);
                }
            }
            mcut
        };
        cuts[start] = cut;
        cut
    }
    pub fn min_cut(s: String) -> i32 {
        Self::min_cut_slice(s.as_bytes(), 0, &mut vec![0; s.len()]) - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_cut() {
        assert_eq!(Solution::min_cut(String::from("aab")), 1);
        assert_eq!(Solution::min_cut(String::from("a")), 0);
        assert_eq!(Solution::min_cut(String::from("ab")), 1);
    }
}
