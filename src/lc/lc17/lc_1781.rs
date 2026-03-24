// https://leetcode.com/problems/sum-of-beauty-of-all-substrings/
// 1781. Sum of Beauty of All Substrings
pub struct Solution;
impl Solution {
    pub fn beauty_sum(s: String) -> i32 {
        let mut ans = 0;
        let s = s.as_bytes();
        for i in 0..s.len() {
            let mut cnt = [0; 26];
            for j in i..s.len() {
                cnt[(s[j] - b'a') as usize] += 1;
                let mut min_cnt = i32::MAX;
                let mut max_cnt = 0;
                for k in 0..26 {
                    if cnt[k] > 0 {
                        min_cnt = min_cnt.min(cnt[k]);
                        max_cnt = max_cnt.max(cnt[k]);
                    }
                }
                ans += max_cnt - min_cnt;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn beauty_sum() {
        assert_eq!(Solution::beauty_sum("aabcb".to_string()), 5);
        assert_eq!(Solution::beauty_sum("aabcbaa".to_string()), 17);
    }
}
