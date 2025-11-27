// https://leetcode.cn/problems/maximum-value-of-an-ordered-triplet-i/
// 2873. Maximum Value of an Ordered Triplet I
pub struct Solution;
impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        super::lc_2874::Solution::maximum_triplet_value(nums)
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
