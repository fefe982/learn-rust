// https://leetcode.com/problems/minimum-moves-to-equal-array-elements/
// 453. Minimum Moves to Equal Array Elements
pub struct Solution;
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable_by_key(|&x| std::cmp::Reverse(x));
        let mut res = 0;
        for i in 1..nums.len() {
            res += i as i32 * (nums[i - 1] - nums[i]);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_moves() {
        assert_eq!(Solution::min_moves(vec![1, 1, 1000000000]), 999999999);
        assert_eq!(Solution::min_moves(vec![1, 2, 3]), 3);
        assert_eq!(Solution::min_moves(vec![1, 1, 1]), 0);
    }
}
