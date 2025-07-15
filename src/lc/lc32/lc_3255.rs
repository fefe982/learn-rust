// https://leetcode.cn/problems/find-the-power-of-k-size-subarrays-ii/
// 3255. Find the Power of K Size Subarrays II
pub struct Solution;
impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        super::lc_3254::Solution::results_array(nums, k)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_the_power_of_k_size_subarrays_i() {
        assert_eq!(
            Solution::results_array(vec![1, 2, 3, 4, 3, 2, 5], 3),
            [3, 4, -1, -1, -1]
        );
        assert_eq!(Solution::results_array(vec![2, 2, 2, 2, 2], 4), [-1, -1]);
        assert_eq!(Solution::results_array(vec![3, 2, 3, 2, 3, 2], 2), [-1, 3, -1, 3, -1]);
    }
}
