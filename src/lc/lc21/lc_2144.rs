// https://leetcode.com/problems/minimum-cost-of-buying-candies-with-discount/
// 2144. Minimum Cost of Buying Candies With Discount
pub struct Solution;
impl Solution {
    pub fn minimum_cost(cost: Vec<i32>) -> i32 {
        let mut cost = cost;
        cost.sort_unstable_by(|a, b| b.cmp(a));
        let mut res = 0;
        for i in 0..cost.len() {
            if i % 3 != 2 {
                res += cost[i];
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_cost() {
        assert_eq!(Solution::minimum_cost(vec![1, 2, 3]), 5);
        assert_eq!(Solution::minimum_cost(vec![6, 5, 7, 9, 2, 2]), 23);
        assert_eq!(Solution::minimum_cost(vec![5, 5]), 10);
    }
}
