// https://leetcode.com/problems/minimum-elements-to-add-to-form-a-given-sum/
// 1785. Minimum Elements to Add to Form a Given Sum
pub struct Solution;
impl Solution {
    pub fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {
        let sum = nums.into_iter().fold(0i64, |acc, x| acc + x as i64);
        let diff = (goal as i64 - sum).abs();
        ((diff + limit as i64 - 1) / limit as i64) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_elements() {
        assert_eq!(Solution::min_elements(vec![1, -1, 1], 3, -4), 2);
        assert_eq!(Solution::min_elements(vec![1, -10, 9, 1], 100, 0), 1);
    }
}
