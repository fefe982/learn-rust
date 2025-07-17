// https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together-ii/
// 2134. Minimum Swaps to Group All 1's Together II
pub struct Solution;
impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.iter().sum::<i32>();
        let mut sum = 0;
        for i in 0..n {
            sum += nums[i as usize];
        }
        let mut min = n - sum;
        for i in 1..nums.len() {
            sum += nums[(i + n as usize - 1) % nums.len()] - nums[i - 1];
            min = min.min(n - sum);
        }
        min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_swaps() {
        assert_eq!(Solution::min_swaps(vec![0, 1, 0, 1, 1, 0, 0]), 1);
        assert_eq!(Solution::min_swaps(vec![0, 1, 1, 1, 0, 0, 1, 1, 0]), 2);
        assert_eq!(Solution::min_swaps(vec![1, 1, 0, 0, 1]), 0);
    }
}
