// https://leetcode.com/problems/minimum-cost-to-connect-two-groups-of-points/
// 1595. Minimum Cost to Connect Two Groups of Points
pub struct Solution;
impl Solution {
    pub fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
        let sz1 = cost.len();
        let sz2 = cost[0].len();
        let mut dp = vec![vec![i32::MAX / 2; 1 << sz2]; sz1 + 1];
        dp[0][0] = 0;
        for i in 1..=sz1 {
            for s in 1..1 << sz2 {
                for k in 0..sz2 {
                    let is = 1 << k;
                    if s & is == 0 {
                        continue;
                    }
                    let sk = s ^ is;
                    dp[i][s] = dp[i][s]
                        .min(dp[i - 1][sk].min(dp[i - 1][s]).min(dp[i][sk]) + cost[i - 1][k]);
                }
            }
        }
        dp[sz1][(1 << sz2) - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn connect_two_groups() {
        assert_eq!(
            Solution::connect_two_groups(vec_vec![[15, 96], [36, 2]]),
            17
        );
        assert_eq!(
            Solution::connect_two_groups(vec_vec![[1, 3, 5], [4, 1, 1], [1, 5, 3]]),
            4
        );
        assert_eq!(
            Solution::connect_two_groups(vec_vec![
                [2, 5, 1],
                [3, 4, 7],
                [8, 1, 2],
                [6, 2, 4],
                [3, 8, 8]
            ]),
            10
        );
    }
}
