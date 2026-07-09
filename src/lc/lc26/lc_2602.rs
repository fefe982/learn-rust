// https://leetcode.com/problems/minimum-operations-to-make-all-array-elements-equal/
// 2602. Minimum Operations to Make All Array Elements Equal II
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
        let mut nums = nums;
        nums.sort();
        let mut sum = Vec::with_capacity(nums.len() + 1);
        sum.push(0);
        for i in 0..nums.len() {
            sum.push(sum[i] + nums[i] as i64);
        }
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            let l = nums.partition_point(|&x| x < q);
            ans.push(sum[nums.len()] - sum[l] * 2 + q as i64 * (l as i64 * 2 - nums.len() as i64));
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations(vec![3, 1, 6, 8], vec![1, 5]), vec![14, 10]);
        assert_eq!(Solution::min_operations(vec![2, 9, 6, 3], vec![10]), vec![20]);
    }
}
