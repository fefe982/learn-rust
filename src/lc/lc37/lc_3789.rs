// https://leetcode.com/problems/minimum-cost-to-acquire-required-items/
// 3789. Minimum Cost to Acquire Required Items
pub struct Solution;
impl Solution {
    pub fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
        let need_both = need1.min(need2) as i64;
        let need1 = need1 as i64 - need_both;
        let need2 = need2 as i64 - need_both;
        let cost_both = (cost1 + cost2).min(cost_both) as i64;
        let cost1 = cost_both.min(cost1 as i64);
        let cost2 = cost_both.min(cost2 as i64);
        need_both * cost_both + cost1 * need1 + cost2 * need2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_cost() {
        assert_eq!(Solution::minimum_cost(3, 2, 1, 3, 2), 3);
        assert_eq!(Solution::minimum_cost(5, 4, 15, 2, 3), 22);
        assert_eq!(Solution::minimum_cost(5, 4, 15, 0, 0), 0);
    }
}
