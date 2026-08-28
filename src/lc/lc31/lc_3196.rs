// https://leetcode.com/problems/maximize-total-cost-of-alternating-subarrays/
// 3196. Maximize Total Cost of Alternating Subarrays
pub struct Solution;
impl Solution {
    pub fn maximum_total_cost(nums: Vec<i32>) -> i64 {
        let mut qp = nums[0] as i64;
        let mut qm = nums[0] as i64;
        for i in 1..nums.len() {
            let nqp = qp.max(qm) + nums[i] as i64;
            let nqm = qp - nums[i] as i64;
            qp = nqp;
            qm = nqm;
        }
        qp.max(qm)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_total_cost() {
        assert_eq!(Solution::maximum_total_cost(vec![1, -2, 3, 4]), 10);
        assert_eq!(Solution::maximum_total_cost(vec![1, -1, 1, -1]), 4);
        assert_eq!(Solution::maximum_total_cost(vec![0]), 0);
        assert_eq!(Solution::maximum_total_cost(vec![1, -1]), 2);
    }
}
