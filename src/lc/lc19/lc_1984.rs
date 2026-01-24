// https://leetcode.com/problems/minimum-difference-between-highest-and-lowest-of-k-scores/
// 1984. Minimum Difference Between Highest and Lowest of K Scores
pub struct Solution;
impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        if k == 1 {
            return 0;
        }
        let mut nums = nums;
        nums.sort_unstable();
        let k = k as usize;
        let mut min = i32::MAX;
        for i in k - 1..nums.len() {
            min = min.min(nums[i] - nums[i + 1 - k]);
        }
        min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_difference() {
        assert_eq!(Solution::minimum_difference(vec![90], 1), 0);
        assert_eq!(Solution::minimum_difference(vec![9, 4, 1, 7], 2), 2);
    }
}
