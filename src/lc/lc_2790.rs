// https://leetcode.com/problems/maximum-number-of-groups-with-increasing-length/
// 2790. Maximum Number of Groups With Increasing Length
pub struct Solution;
impl Solution {
    pub fn max_increasing_groups(usage_limits: Vec<i32>) -> i32 {
        let mut usage_limits = usage_limits;
        usage_limits.sort_unstable();
        let mut res = 0;
        let mut sum = 0;
        let mut bound = 1;
        for i in 0..usage_limits.len() {
            sum += usage_limits[i];
            if sum >= bound {
                res += 1;
                bound += res + 1;
            }
        }
        res
    }
}
mod tests {
    use super::*;
    #[test]
    fn test_max_increasing_groups() {
        assert_eq!(Solution::max_increasing_groups(vec![1, 2, 5]), 3);
        assert_eq!(Solution::max_increasing_groups(vec![2, 1, 2]), 2);
    }
}
