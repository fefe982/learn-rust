// https://leetcode.com/problems/longest-ideal-subsequence/
// 2370. Longest Ideal Subsequence
pub struct Solution;
impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let mut lv = vec![0; 26];
        let mut longest = 0;
        let k = k as usize;
        for c in s.chars() {
            let i = (c as u8 - b'a') as usize;
            let low = (i + 26 - k).max(26) - 26;
            let high = (i + k).min(25);
            let l = lv[low..=high].iter().max().unwrap() + 1;
            lv[i] = l;
            longest = longest.max(l)
        }
        longest as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_ideal_string() {
        assert_eq!(Solution::longest_ideal_string("acfgbd".to_string(), 2), 4);
        assert_eq!(Solution::longest_ideal_string("abcd".to_string(), 3), 4);
    }
}
