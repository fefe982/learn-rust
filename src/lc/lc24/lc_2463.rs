// https://leetcode.com/problems/minimum-total-distance-traveled/
// 2463. Minimum Total Distance Traveled
pub struct Solution;
impl Solution {
    pub fn minimum_total_distance(robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
        let mut robot = robot;
        robot.sort();
        let mut factory = factory;
        factory.sort();
        let mut dp = vec![vec![i64::MAX; robot.len() + 1]; factory.len() + 1];
        for i in 0..factory.len() {
            dp[i][0] = 0;
        }
        for i in 0..robot.len() {
            for j in 0..factory.len() {
                let fpos = factory[j][0];
                let fcap = factory[j][1];
                let mut d = 0;
                dp[j + 1][i + 1] = dp[j][i + 1];
                for k in 1..=(i as usize + 1).min(fcap as usize) {
                    d += (robot[i + 1 - k] - fpos).abs() as i64;
                    if dp[j][i + 1 - k] != i64::MAX {
                        dp[j + 1][i + 1] = dp[j + 1][i + 1].min(dp[j][i + 1 - k] + d);
                    }
                }
            }
        }
        dp[factory.len()][robot.len()]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_minimum_total_distance() {
        assert_eq!(
            Solution::minimum_total_distance(
                vec![9, 11, 99, 101],
                vec_vec![[10, 1], [7, 1], [14, 1], [100, 1], [96, 1], [103, 1]]
            ),
            6
        );
        assert_eq!(
            Solution::minimum_total_distance(vec![0, 4, 6], vec_vec![[2, 2], [6, 2]]),
            4
        );
        assert_eq!(
            Solution::minimum_total_distance(vec![1, -1], vec_vec![[-2, 1], [2, 1]]),
            2
        );
    }
}
