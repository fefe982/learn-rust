// https://leetcode.com/problems/count-beautiful-substrings-ii/
// 2949. Count Beautiful Substrings II
pub struct Solution;
impl Solution {
    pub fn beautiful_substrings(s: String, k: i32) -> i64 {
        let mut len = 1;
        while len * len % k != 0 {
            len += 1;
        }
        len *= 2;
        let len = len as usize;
        let mut cnt = vec![std::collections::HashMap::<i32, i32>::new(); len];
        cnt[0].insert(0, 1);
        let mut n = 0;
        let mut ans = 0;
        for (idx, c) in s.chars().enumerate() {
            if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                n += 1;
            } else {
                n -= 1;
            }
            let i = (idx + 1) % len;
            let c = cnt[i].entry(n).or_default();
            ans += *c as i64;
            *c += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_beautiful_substrings() {
        assert_eq!(Solution::beautiful_substrings("baeyh".to_string(), 2), 2);
        assert_eq!(Solution::beautiful_substrings("abba".to_string(), 1), 3);
        assert_eq!(Solution::beautiful_substrings("bcdf".to_string(), 1), 0);
    }
}
