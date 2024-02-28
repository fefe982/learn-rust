// https://leetcode.com/problems/make-costs-of-paths-equal-in-a-binary-tree/
// 2673. Make Costs of Paths Equal in a Binary Tree
pub struct Solution;
impl Solution {
    pub fn min_increments(n: i32, cost: Vec<i32>) -> i32 {
        let mut cost = cost;
        let mut res = 0;
        for i in (1..=cost.len() / 2).rev() {
            let c1 = cost[i * 2];
            let c2 = cost[i * 2 - 1];
            res += c1.max(c2) - c1.min(c2);
            cost[i - 1] += c1.max(c2);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_increments() {
        assert_eq!(Solution::min_increments(7, vec![1, 5, 2, 2, 3, 3, 1]), 6);
        assert_eq!(Solution::min_increments(3, vec![5, 3, 3]), 6);
    }
}
