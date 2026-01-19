// https://leetcode.com/problems/construct-the-minimum-bitwise-array-i/
// 3314. Construct the Minimum Bitwise Array I
pub struct Solution;
impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        super::lc_3315::Solution::min_bitwise_array(nums)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_bitwise_array() {
        assert_eq!(Solution::min_bitwise_array(vec![2, 3, 5, 7]), [-1, 1, 4, 3]);
        assert_eq!(Solution::min_bitwise_array(vec![11, 13, 31]), [9, 12, 15]);
    }
}
