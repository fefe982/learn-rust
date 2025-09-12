// https://leetcode.com/problems/find-most-frequent-vowel-and-consonant/
// 3541. Find Most Frequent the Vowel or Consonant in Array
pub struct Solution;
impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut cnt = vec![0; 26];
        let mut mv = 0;
        let mut mc = 0;
        for &c in s.as_bytes() {
            let i = (c - b'a') as usize;
            cnt[i] += 1;
            if c == b'a' || c == b'e' || c == b'i' || c == b'o' || c == b'u' {
                mv = mv.max(cnt[i]);
            } else {
                mc = mc.max(cnt[i]);
            }
        }
        mv + mc
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_freq_sum() {
        assert_eq!(Solution::max_freq_sum("successes".to_string()), 6);
        assert_eq!(Solution::max_freq_sum("aeiaeia".to_string()), 3);
    }
}
