// https://leetcode.com/problems/minimum-length-of-string-after-operations/
// 3223. Minimum Length of String After Operations
pub struct Solution;
impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut cnt = [0; 26];
        for c in s.chars() {
            cnt[c as usize - 'a' as usize] += 1;
        }
        cnt.into_iter().fold(0, |acc, x| {
            if x == 0 {
                acc
            } else if x % 2 == 1 {
                acc + 1
            } else {
                acc + 2
            }
        })
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_minimum_length() {
        assert_eq!(Solution::minimum_length("abaacbcbb".to_string()), 5);
        assert_eq!(Solution::minimum_length("aa".to_string()), 2);
    }
}
