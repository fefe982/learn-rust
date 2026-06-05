// https://leetcode.com/problems/left-and-right-sum-differences/
// 2574. Left and Right Sum Differences
pub struct Solution;
impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut left_sum = 0;
        let mut right_sum: i32 = nums.iter().sum();
        let mut ans = Vec::new();
        for n in nums {
            right_sum -= n;
            ans.push((left_sum - right_sum).abs());
            left_sum += n;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_left_right_difference() {
        assert_eq!(Solution::left_right_difference(vec![10, 4, 8, 3]), vec![15, 1, 11, 22]);
        assert_eq!(Solution::left_right_difference(vec![1]), vec![0]);
    }
}
