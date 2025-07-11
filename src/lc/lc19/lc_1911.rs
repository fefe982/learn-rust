// https://leetcode.com/problems/maximum-alternating-subsequence-sum/
// 1911. Maximum Alternating Subsequence Sum
pub struct Solution;
impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut po = nums[0] as i64;
        let mut pe = 0;
        for n in nums.into_iter().skip(1) {
            let n = n as i64;
            let no = (pe + n).max(po);
            let ne = (po - n).max(pe);
            pe = ne;
            po = no;
        }
        po.max(pe)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_alternating_sum() {
        assert_eq!(Solution::max_alternating_sum(vec![4, 2, 5, 3]), 7);
        assert_eq!(Solution::max_alternating_sum(vec![5, 6, 7, 8]), 8);
        assert_eq!(Solution::max_alternating_sum(vec![6, 2, 1, 2, 4, 5]), 10)
    }
}
