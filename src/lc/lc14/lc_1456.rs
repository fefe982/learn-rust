// https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/
// 1456. Maximum Number of Vowels in a Substring of Given Length
pub struct Solution;
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let vowels: std::collections::HashSet<u8> = "aeiou".as_bytes().iter().cloned().collect();
        let mut cnt = 0;
        let s = s.as_bytes();
        let k = k as usize;
        for i in 0..k {
            if vowels.contains(&s[i]) {
                cnt += 1;
            }
        }
        let mut max = cnt;
        for i in k..s.len() {
            if max == k {
                return k as i32;
            }
            if vowels.contains(&s[i - k]) {
                cnt -= 1;
            }
            if vowels.contains(&s[i]) {
                cnt += 1;
            }
            max = std::cmp::max(max, cnt);
        }
        max as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_vowels() {
        assert_eq!(Solution::max_vowels(String::from("abciiidef"), 3), 3);
        assert_eq!(Solution::max_vowels(String::from("aeiou"), 2), 2);
        assert_eq!(Solution::max_vowels(String::from("leetcode"), 3), 2);
    }
}
