// https://leetcode.com/problems/find-pivot-index/
// 724. Find Pivot Index
pub struct Solution;
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut right = nums.iter().sum::<i32>();
        let mut left = 0;
        for (i, n) in nums.into_iter().enumerate() {
            right -= n;
            if left == right {
                return i as i32;
            }
            left += n;
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pivot_index() {
        assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
        assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
        assert_eq!(Solution::pivot_index(vec![2, 1, -1]), 0);
    }
}
