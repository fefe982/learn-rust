// https://leetcode.com/problems/count-subarrays-with-median-k/
// 2488. Count Subarrays With Median K
pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut idx_k = 0;
        for (idx, num) in nums.iter().enumerate() {
            if *num == k {
                idx_k = idx;
                break;
            }
        }
        let mut sum_map_low: HashMap<i32, i32> = HashMap::new();
        let mut sum_map_high: HashMap<i32, i32> = HashMap::new();
        let mut sum = 0;
        let mut total = 0;
        sum_map_low.insert(0, 1);
        for num in nums[0..idx_k].iter() {
            if *num < k {
                sum -= 1;
            } else if *num > k {
                sum += 1;
            }
            if let Some(c) = sum_map_low.get_mut(&sum) {
                *c += 1;
            } else {
                sum_map_low.insert(sum, 1);
            }
        }
        for num in nums[idx_k..].iter() {
            if *num < k {
                sum -= 1;
            } else if *num > k {
                sum += 1;
            }
            if let Some(c) = sum_map_high.get_mut(&sum) {
                *c += 1;
            } else {
                sum_map_high.insert(sum, 1);
            }
        }
        for (k_low, c_low) in sum_map_low.iter() {
            if let Some(c_high) = sum_map_high.get(k_low) {
                total += *c_low * *c_high
            }
            if let Some(c_high) = sum_map_high.get(&(1 + k_low)) {
                total += *c_low * *c_high
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_subarrays() {
        assert_eq!(Solution::count_subarrays(vec![4, 1, 3, 2], 1), 3);
        assert_eq!(Solution::count_subarrays(vec![3, 2, 1, 4, 5], 4), 3);
        assert_eq!(Solution::count_subarrays(vec![2, 3, 1], 3), 1);
    }
}
