// https://leetcode.com/problems/maximum-profit-from-trading-stocks-with-discounts/
// 3562. Maximum Profit From Trading Stocks With Discounts
pub struct Solution;
impl Solution {
    fn dfs(
        g: &Vec<Vec<usize>>,
        present: &Vec<i32>,
        future: &Vec<i32>,
        budget: usize,
        i: usize,
    ) -> (Vec<(i32, i32)>, usize) {
        let mut max_cost = present[i] as usize;
        let mut child_cost_sum = vec![(0, 0); budget + 1];
        for &j in &g[i] {
            let (profit_child, max_cost_child) = Solution::dfs(g, present, future, budget, j);
            max_cost += max_cost_child;
            for k in (0..=budget).rev() {
                for l in 0..=k.min(max_cost_child) {
                    child_cost_sum[k].0 = child_cost_sum[k].0.max(child_cost_sum[k - l].0 + profit_child[l].0);
                    child_cost_sum[k].1 = child_cost_sum[k].1.max(child_cost_sum[k - l].1 + profit_child[l].1);
                }
            }
        }
        let mut this_profit = vec![(0, 0); budget + 1];
        let cost = present[i] as usize;
        let half_cost = cost / 2;
        for j in 0..=budget {
            this_profit[j].0 = child_cost_sum[j].0;
            if j >= cost {
                this_profit[j].0 = this_profit[j]
                    .0
                    .max(child_cost_sum[j - cost].1 - cost as i32 + future[i]);
            }
            this_profit[j].1 = child_cost_sum[j].0;
            if j >= half_cost {
                this_profit[j].1 = this_profit[j]
                    .1
                    .max(child_cost_sum[j - half_cost].1 - half_cost as i32 + future[i]);
            }
        }
        (this_profit, max_cost)
    }
    pub fn max_profit(n: i32, present: Vec<i32>, future: Vec<i32>, hierarchy: Vec<Vec<i32>>, budget: i32) -> i32 {
        let mut g = vec![vec![]; n as usize + 1];
        for h in hierarchy {
            g[h[0] as usize - 1].push(h[1] as usize - 1);
        }
        let (profit, _) = Solution::dfs(&g, &present, &future, budget as usize, 0);
        profit[budget as usize].0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_profit() {
        assert_eq!(Solution::max_profit(2, vec![1, 2], vec![4, 3], vec_vec![[1, 2]], 3), 5);
        assert_eq!(Solution::max_profit(2, vec![3, 4], vec![5, 8], vec_vec![[1, 2]], 4), 4);
        assert_eq!(
            Solution::max_profit(3, vec![4, 6, 8], vec![7, 9, 11], vec_vec![[1, 2], [1, 3]], 10),
            10
        );
        assert_eq!(
            Solution::max_profit(3, vec![5, 2, 3], vec![8, 5, 6], vec_vec![[1, 2], [2, 3]], 7),
            12
        );
    }
}
