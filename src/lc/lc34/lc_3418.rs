// https://leetcode.com/problems/maximum-amount-of-money-robot-can-earn/
// 3418. Maximum Amount of Money the Robot Can Earn
pub struct Solution;
impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let m = coins.len();
        let n = coins[0].len();
        let neg_inf = i64::MIN / 4;

        // prev[j][k]: best profit at current processed row-1, column j, using k neutralizations.
        let mut prev = vec![[neg_inf; 3]; n];

        for i in 0..m {
            let mut curr = vec![[neg_inf; 3]; n];
            for j in 0..n {
                let val = coins[i][j] as i64;
                let mut best_from = [neg_inf; 3];

                if i == 0 && j == 0 {
                    best_from[0] = 0;
                } else {
                    if i > 0 {
                        for k in 0..3 {
                            best_from[k] = best_from[k].max(prev[j][k]);
                        }
                    }
                    if j > 0 {
                        for k in 0..3 {
                            best_from[k] = best_from[k].max(curr[j - 1][k]);
                        }
                    }
                }

                for used in 0..3 {
                    let base = best_from[used];
                    if base == neg_inf {
                        continue;
                    }

                    if val >= 0 {
                        curr[j][used] = curr[j][used].max(base + val);
                    } else {
                        // Do not neutralize this robber.
                        curr[j][used] = curr[j][used].max(base + val);
                        // Neutralize this robber if we still can.
                        if used < 2 {
                            curr[j][used + 1] = curr[j][used + 1].max(base);
                        }
                    }
                }
            }
            prev = curr;
        }

        *prev[n - 1].iter().max().unwrap() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn maximum_amount() {
        assert_eq!(
            Solution::maximum_amount(vec_vec![[0, 1, -1], [1, -2, 3], [2, -3, 4]]),
            8
        );
        assert_eq!(Solution::maximum_amount(vec_vec![[10, 10, 10], [10, 10, 10]]), 40);
    }
}
