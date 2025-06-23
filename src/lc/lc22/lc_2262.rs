// https://leetcode.com/problems/total-appeal-of-a-string/
// 2262. Total Appeal of A String
pub struct Solution;
impl Solution {
    pub fn appeal_sum(s: String) -> i64 {
        let mut cnt = 0;
        let mut last_pos = vec![0; 26];
        let s = s.as_bytes();
        for (&c, i) in s.iter().zip(1..) {
            let ic = (c - b'a') as usize;
            cnt += (i - last_pos[ic]) * (i - last_pos[ic] - 1) / 2;
            last_pos[ic] = i;
        }
        let mut ans = -cnt;
        let n = s.len() as i64 + 1;
        let full = n * (n - 1) / 2;
        for i in 0..26 {
            if last_pos[i] > 0 {
                ans += full - (n - last_pos[i]) * (n - last_pos[i] - 1) / 2;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_appeal_sum() {
        assert_eq!(Solution::appeal_sum("abbca".to_string()), 28);
        assert_eq!(Solution::appeal_sum("code".to_string()), 20);
    }
}
