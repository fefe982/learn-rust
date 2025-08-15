// https://leetcode.com/problems/maximize-sum-of-array-after-k-negations
// 1005. Maximize Sum Of Array After K Negations
pub struct Solution;
impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut k = k;
        let mut sum = nums.iter().sum();
        for i in 0..nums.len() {
            if k == 0 {
                break;
            }
            if nums[i] < 0 {
                sum -= 2 * nums[i];
                k -= 1;
            } else if nums[i] == 0 {
                k = 0;
                break;
            } else if nums[i] > 0 {
                if k % 2 == 1 {
                    sum -= 2 * nums[i].min(if i > 0 { -nums[i - 1] } else { i32::MAX });
                }
                k = 0;
                break;
            }
        }
        if (k % 2) == 1 {
            sum += 2 * nums[nums.len() - 1];
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn largest_sum_after_k_negations() {
        assert_eq!(Solution::largest_sum_after_k_negations(vec![-4, -2, -3], 4), 5);
        assert_eq!(Solution::largest_sum_after_k_negations(vec![4, 2, 3], 1), 5);
        assert_eq!(Solution::largest_sum_after_k_negations(vec![3, -1, 0, 2], 3), 6);
        assert_eq!(Solution::largest_sum_after_k_negations(vec![2, -3, -1, 5, -4], 2), 13);
    }
}
