// https://leetcode.com/problems/min-cost-climbing-stairs/
// 746. Min Cost Climbing Stairs
pub struct Solution;
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut last1 = 0;
        let mut last2 = 0;
        for c in cost {
            let cur = c + last1.min(last2);
            last2 = last1;
            last1 = cur;
        }
        last1.min(last2)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_cost_climbing_stairs() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
