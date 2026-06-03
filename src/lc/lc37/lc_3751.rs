// https://leetcode.com/problems/total-waviness-of-numbers-in-range-i/description/?envType=daily-question&envId=2026-06-04
// 3751. Total Waviness of Numbers in Range I
pub struct Solution;
impl Solution {
    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
        super::lc_3753::Solution::total_waviness(num1 as i64, num2 as i64) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_total_waviness() {
        assert_eq!(Solution::total_waviness(120, 130), 3);
        assert_eq!(Solution::total_waviness(198, 202), 3);
    }
}
