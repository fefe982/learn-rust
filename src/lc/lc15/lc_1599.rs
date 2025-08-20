// https://leetcode.com/problems/maximum-profit-of-operating-a-centennial-wheel/
// 1599. Maximum Profit of Operating a Centennial Wheel
pub struct Solution;
impl Solution {
    pub fn min_operations_max_profit(
        customers: Vec<i32>,
        boarding_cost: i32,
        running_cost: i32,
    ) -> i32 {
        if running_cost >= boarding_cost * 4 {
            return -1;
        }
        let mut step = 0;
        let mut customer_wait = 0;
        let mut max_profit = 0;
        let mut max_profit_step = -1;
        let mut current_profit = 0;
        for c in customers.iter() {
            step += 1;
            customer_wait += *c;
            let mut this_run = 4;
            if customer_wait < 4 {
                this_run = customer_wait;
            }
            customer_wait -= this_run;
            current_profit += this_run * boarding_cost - running_cost;
            if current_profit > max_profit {
                max_profit = current_profit;
                max_profit_step = step;
            }
        }
        let remain_step = customer_wait / 4;
        current_profit += remain_step * (4 * boarding_cost - running_cost);
        step += remain_step;
        customer_wait -= remain_step * 4;
        if customer_wait * boarding_cost > running_cost {
            step += 1;
            current_profit += customer_wait * boarding_cost - running_cost;
        }
        if current_profit > max_profit {
            max_profit_step = step
        }
        max_profit_step
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations_max_profit() {
        assert_eq!(Solution::min_operations_max_profit(vec![8, 3], 5, 6), 3);
        assert_eq!(Solution::min_operations_max_profit(vec![10, 9, 6], 6, 4), 7);
        assert_eq!(
            Solution::min_operations_max_profit(vec![3, 4, 0, 5, 1], 1, 92),
            -1
        );
    }
}
