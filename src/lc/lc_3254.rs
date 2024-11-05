// https://leetcode.cn/problems/find-the-power-of-k-size-subarrays-i/
// 3254. Find the Power of K Size Subarrays I
pub struct Solution;
impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut res = vec![-1; nums.len() - k + 1];
        let mut cnt = 0;
        for i in 1..k {
            if nums[i - 1] + 1 == nums[i] {
                cnt += 1;
            }
        }
        if cnt == k - 1 {
            res[0] = nums[k - 1];
        }
        for i in k..nums.len() {
            if nums[i - 1] + 1 == nums[i] {
                cnt += 1;
            }
            if nums[i - k] + 1 == nums[i - k + 1] {
                cnt -= 1;
            }
            if cnt == k - 1 {
                res[i - k + 1] = nums[i];
            }
        }
        res
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
