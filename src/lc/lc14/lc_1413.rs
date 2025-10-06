// https://leetcode.com/problems/minimum-value-to-get-positive-step-by-step-sum/
// 1413. Minimum Value to Get Positive Step by Step Sum
pub struct Solution;
impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((0, 0), |(min_sum, cur_sum), x| {
                let new_sum = cur_sum + x;
                (min_sum.min(new_sum), new_sum)
            })
            .0
            .abs()
            + 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_start_value() {
        assert_eq!(Solution::min_start_value(vec![-3, 2, -3, 4, 2]), 5);
        assert_eq!(Solution::min_start_value(vec![1, 2]), 1);
        assert_eq!(Solution::min_start_value(vec![1, -2, -3]), 5);
    }
}
