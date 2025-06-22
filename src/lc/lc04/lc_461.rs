// https://leetcode.com/problems/hamming-distance/
// 461. Hamming Distance
pub struct Solution;
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hamming_distance() {
        assert_eq!(Solution::hamming_distance(1, 4), 2);
        assert_eq!(Solution::hamming_distance(3, 1), 1);
    }
}
