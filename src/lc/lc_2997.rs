// https://leetcode.com/problems/minimum-number-of-operations-to-make-array-xor-equal-to-k
// 2997. Minimum Number of Operations to Make Array XOR Equal to K
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.into_iter().fold(k, |acc, x| acc ^ x).count_ones() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations(vec![2, 1, 3, 4], 1), 2);
        assert_eq!(Solution::min_operations(vec![2, 0, 2, 0], 0), 0);
    }
}
