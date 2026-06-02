// https://leetcode.com/problems/minimum-average-difference/
// 2256. Minimum Average Difference
pub struct Solution;
impl Solution {
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().map(|&x| x as i64).sum::<i64>();
        let mut mindiff = sum / nums.len() as i64;
        let mut prefix_sum = 0i64;
        let mut minidx = nums.len() - 1;
        for i in 1..nums.len() {
            prefix_sum += nums[i - 1] as i64;
            let suffix_sum = sum - prefix_sum;
            let avg_diff = (prefix_sum / i as i64 - suffix_sum / (nums.len() as i64 - i as i64)).abs();
            if avg_diff < mindiff {
                mindiff = avg_diff;
                minidx = i - 1;
            } else if avg_diff == mindiff {
                minidx = minidx.min(i - 1);
            }
        }
        minidx as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_average_difference() {
        assert_eq!(Solution::minimum_average_difference(vec![2, 5, 3, 9, 5, 3]), 3);
        assert_eq!(Solution::minimum_average_difference(vec![0]), 0);
    }
}
