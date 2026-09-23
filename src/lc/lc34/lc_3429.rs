// https://leetcode.com/problems/paint-house-iv/description/?envType=problem-list-v2&envId=2sh4gvmh
// 3429. Paint House IV
pub struct Solution;
impl Solution {
    fn walk(cost: &Vec<Vec<i32>>, dp: &mut Vec<Vec<Vec<i64>>>, i: usize, l: usize, r: usize) -> i64 {
        let n = cost.len();
        if i >= n / 2 {
            return 0;
        }
        if dp[i][l][r] != -1 {
            return dp[i][l][r];
        }
        let mut ans = i64::MAX;
        for nl in 0..3 {
            if nl == l {
                continue;
            }
            for nr in 0..3 {
                if nr == r || nl == nr {
                    continue;
                }
                ans = ans.min(cost[i][nl] as i64 + cost[n - i - 1][nr] as i64 + Self::walk(cost, dp, i + 1, nl, nr));
            }
        }
        dp[i][l][r] = ans;
        ans
    }
    pub fn min_cost(n: i32, cost: Vec<Vec<i32>>) -> i64 {
        let mut dp = vec![vec![vec![-1; 4]; 4]; n as usize];
        Self::walk(&cost, &mut dp, 0, 3, 3)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_cost() {
        assert_eq!(
            Solution::min_cost(4, vec_vec![[3, 5, 7], [6, 2, 9], [4, 8, 1], [7, 3, 5]]),
            9
        );
        assert_eq!(
            Solution::min_cost(
                6,
                vec_vec![[2, 4, 6], [5, 3, 8], [7, 1, 9], [4, 6, 2], [3, 5, 7], [8, 2, 4]]
            ),
            18
        );
    }
}
