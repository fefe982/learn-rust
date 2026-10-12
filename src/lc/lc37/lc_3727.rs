// https://leetcode.com/problems/maximum-alternating-sum-of-squares/
// 3727. Maximum Alternating Sum of Squares
pub struct Solution;
impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        nums.sort_by_key(|&x| x.abs());
        let mut res = 0;
        for i in 0..nums.len() / 2 {
            let n = nums[i] as i64;
            res -= n * n;
        }
        for i in (nums.len() / 2)..nums.len() {
            let n = nums[i] as i64;
            res += n * n;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_alternating_sum() {
        assert_eq!(Solution::max_alternating_sum(vec![1, 2, 3]), 12);
        assert_eq!(Solution::max_alternating_sum(vec![1, -1, 2, -2, 3, -3]), 16);
    }
}
