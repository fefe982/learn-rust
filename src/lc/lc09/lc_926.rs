// https://leetcode.com/problems/flip-string-to-monotone-increasing/
// 926. Flip String to Monotone Increasing
pub struct Solution;
impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let mut nz = 0;
        let mut min_flip = 0;
        let mut flip = 0;
        for c in s.chars() {
            if c == '1' {
                flip += 1;
            } else {
                flip -= 1;
                nz += 1;
                min_flip = min_flip.min(flip);
            }
        }
        min_flip + nz
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_flips_mono_incr() {
        assert_eq!(Solution::min_flips_mono_incr("00110".to_string()), 1);
        assert_eq!(Solution::min_flips_mono_incr("010110".to_string()), 2);
        assert_eq!(Solution::min_flips_mono_incr("00011000".to_string()), 2);
    }
}
