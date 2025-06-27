// https://leetcode.com/problems/profitable-schemes/
// 879. Profitable Schemes
use std::cmp;
pub struct Solution;

impl Solution {
    fn recurse(
        i: usize,
        n: i32,
        min_profit: i32,
        group: &Vec<i32>,
        profit: &Vec<i32>,
        cache: &mut Vec<Vec<Vec<i32>>>,
    ) -> i32 {
        if n < 0 {
            return 0;
        }
        if n == 0 {
            if min_profit == 0 {
                return 1;
            } else {
                return 0;
            }
        }
        if i == group.len() {
            if n > 0 && min_profit == 0 {
                return 1;
            } else {
                return 0;
            }
        }
        if cache[i][n as usize][min_profit as usize] != -1 {
            return cache[i][n as usize][min_profit as usize];
        }
        let mut sum = Self::recurse(i + 1, n, min_profit, group, profit, cache);
        sum =
            (sum + Self::recurse(
                i + 1,
                n - group[i],
                cmp::max(0, min_profit - profit[i]),
                group,
                profit,
                cache,
            )) % 1_000_000_007;
        cache[i][n as usize][min_profit as usize] = sum;
        sum
    }
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut cache =
            vec![vec![vec![-1; (min_profit + 1) as usize]; (n + 1) as usize]; group.len()];
        Self::recurse(0usize, n, min_profit, &group, &profit, &mut cache)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn profitable_schemes() {
        assert_eq!(
            Solution::profitable_schemes(5, 3, vec![2, 2], vec![2, 3]),
            2
        );
        assert_eq!(
            Solution::profitable_schemes(10, 5, vec![2, 3, 5], vec![6, 7, 8]),
            7
        );
    }
}
