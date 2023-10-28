// https://leetcode.com/problems/predict-the-winner/
// 486. Predict the Winner
pub struct Solution;
impl Solution {
    fn margin(
        nums: &Vec<i32>,
        start: usize,
        len: usize,
        cache: &mut std::collections::HashMap<(usize, usize), i32>,
    ) -> i32 {
        if len == 1 {
            return nums[start];
        }
        if let Some(&m) = cache.get(&(start, len)) {
            return m;
        }
        let m = (nums[start] - Self::margin(nums, start + 1, len - 1, cache))
            .max(nums[start + len - 1] - Self::margin(nums, start, len - 1, cache));
        cache.insert((start, len), m);
        m
    }
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        Self::margin(&nums, 0, nums.len(), &mut std::collections::HashMap::new()) >= 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn predict_the_winner() {
        assert_eq!(Solution::predict_the_winner(vec![1, 5, 2]), false);
        assert_eq!(Solution::predict_the_winner(vec![1, 5, 233, 7]), true);
    }
}
