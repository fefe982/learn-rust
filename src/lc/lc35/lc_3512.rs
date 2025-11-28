// https://leetcode.com/problems/minimum-operations-to-make-array-sum-divisible-by-k/
// 3512. Minimum Operations to Make Array Sum Divisible by K
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.into_iter().sum::<i32>() % k
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations(vec![3, 9, 7], 5), 4);
        assert_eq!(Solution::min_operations(vec![4, 1, 3], 4), 0);
        assert_eq!(Solution::min_operations(vec![3, 2], 6), 5);
    }
}
