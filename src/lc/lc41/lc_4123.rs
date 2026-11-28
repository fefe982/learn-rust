// https://leetcode.com/problems/minimum-cost-to-delete-array/
// 4123. Minimum Cost to Delete Array
pub struct Solution;
impl Solution {
    pub fn min_cost(nums: Vec<i32>) -> i64 {
        let l = nums.len();
        let mut sum = nums[0] as i64 + nums[l - 1] as i64;
        let mut max = i32::MIN;
        let mut min = i32::MAX;
        let mut c = 0;
        for i in 1..l - 1 {
            let n = nums[i];
            if n < 0 {
                sum += n as i64;
                c += 1;
                max = max.max(n);
            } else {
                min = min.min(n);
            }
        }
        if c % 2 == 1 {
            if min == i32::MAX || max + min > 0 {
                sum -= max as i64;
            } else {
                sum += min as i64;
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_cost() {
        assert_eq!(Solution::min_cost(vec![2, -7, 3, -5]), -7);
        assert_eq!(Solution::min_cost(vec![4, 0, -5, 2]), 1);
        assert_eq!(Solution::min_cost(vec![7, -8, 1]), 8);
    }
}
