// https://leetcode.cn/problems/maximum-value-of-an-ordered-triplet-ii/
// 2874. Maximum Value of an Ordered Triplet II
pub struct Solution;
impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut max = nums[0];
        let mut dmax = nums[0] - nums[1];
        let mut res = 0;
        for i in 2..nums.len() {
            res = res.max(dmax as i64 * nums[i] as i64);
            max = max.max(nums[i - 1]);
            dmax = dmax.max(max - nums[i]);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_triplet_value() {
        assert_eq!(Solution::maximum_triplet_value(vec![12, 6, 1, 2, 7]), 77);
        assert_eq!(Solution::maximum_triplet_value(vec![1, 10, 3, 4, 19]), 133);
        assert_eq!(Solution::maximum_triplet_value(vec![1, 2, 3]), 0);
    }
}
