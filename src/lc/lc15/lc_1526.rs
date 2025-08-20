// https://leetcode.com/problems/minimum-number-of-increments-on-subarrays-to-form-a-target-array/
// 1526. Minimum Number of Increments on Subarrays to Form a Target Array
pub struct Solution;
impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        target
            .into_iter()
            .fold((0, 0), |(last, sum), x| (x, sum + (x - last).max(0)))
            .1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_number_operations() {
        assert_eq!(Solution::min_number_operations(vec![1, 2, 3, 2, 1]), 3);
        assert_eq!(Solution::min_number_operations(vec![3, 1, 1, 2]), 4);
        assert_eq!(Solution::min_number_operations(vec![3, 1, 5, 4, 2]), 7);
    }
}
