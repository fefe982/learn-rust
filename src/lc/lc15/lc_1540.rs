// https://leetcode.com/problems/can-convert-string-in-k-moves/
// 1540. Can Convert String in K Moves
pub struct Solution;
impl Solution {
    pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut cnt = vec![0; 26];
        let mut max = 0;
        for (sc, tc) in s.chars().zip(t.chars()) {
            let sc = (sc as u8 - b'a') as usize;
            let tc = (tc as u8 - b'a') as usize;
            if sc != tc {
                let idx = (tc + 26 - sc) % 26;
                max = max.max(cnt[idx] * 26 + idx);
                cnt[idx] += 1;
            }
        }
        max as i32 <= k
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_convert_string() {
        assert_eq!(
            Solution::can_convert_string("input".to_string(), "ouput".to_string(), 9),
            true
        );
        assert_eq!(
            Solution::can_convert_string("abc".to_string(), "bcd".to_string(), 10),
            false
        );
        assert_eq!(
            Solution::can_convert_string("aab".to_string(), "bbb".to_string(), 27),
            true
        );
    }
}
