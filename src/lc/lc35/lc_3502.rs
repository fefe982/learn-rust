// https://leetcode.com/problems/minimum-cost-to-reach-every-position/
// 3502. Minimum Cost to Reach Every Position
pub struct Solution;
impl Solution {
    pub fn min_costs(cost: Vec<i32>) -> Vec<i32> {
        let mut cost = cost;
        for i in 1..cost.len() {
            cost[i] = cost[i].min(cost[i - 1]);
        }
        cost
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_costs() {
        assert_eq!(Solution::min_costs(vec![5, 3, 4, 1, 3, 2]), vec![5, 3, 3, 1, 1, 1]);
        assert_eq!(Solution::min_costs(vec![1, 2, 4, 6, 7]), vec![1, 1, 1, 1, 1]);
    }
}
