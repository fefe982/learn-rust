// https://leetcode.com/problems/minimum-positive-sum-subarray/
// 3364. Minimum Positive Sum Subarray
pub struct Solution;
impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut min = i32::MAX;
        let l = l as usize;
        let r = r as usize;
        let mut s = 0;
        for i in 0..l - 1 {
            s += nums[i];
        }
        for i in l..=r {
            let mut sum = s;
            for j in i - 1..nums.len() {
                sum += nums[j];
                if sum > 0 {
                    min = min.min(sum);
                }
                sum -= nums[j + 1 - i];
            }
            s += nums[i - 1];
        }
        if min == i32::MAX {
            -1
        } else {
            min
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_sum_subarray() {
        assert_eq!(Solution::minimum_sum_subarray(vec![4, -10], 1, 1), 4);
        assert_eq!(Solution::minimum_sum_subarray(vec![3, -2, 1, 4], 2, 3), 1);
        assert_eq!(Solution::minimum_sum_subarray(vec![-2, 2, -3, 1], 2, 3), -1);
        assert_eq!(Solution::minimum_sum_subarray(vec![1, 2, 3, 4], 2, 4), 3);
    }
}
