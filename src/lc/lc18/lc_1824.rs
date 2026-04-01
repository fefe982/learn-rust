// https://leetcode.com/problems/minimum-sideway-jumps/
// 1824. Minimum Sideway Jumps
pub struct Solution;
impl Solution {
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        const INF: i32 = 1_000_000_000;

        // dp[lane] = min side jumps to stand at current point on this lane.
        // Lanes 1..=3 are stored at indices 0..=2.
        let mut dp = [1, 0, 1];

        for &obs in obstacles.iter().skip(1) {
            if obs != 0 {
                dp[(obs - 1) as usize] = INF;
            }

            let best = dp[0].min(dp[1]).min(dp[2]);

            for lane in 0..3 {
                if obs != (lane as i32 + 1) {
                    dp[lane] = dp[lane].min(best + 1);
                }
            }
        }

        dp[0].min(dp[1]).min(dp[2])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_side_jumps() {
        assert_eq!(Solution::min_side_jumps(vec![0, 1, 2, 3, 0]), 2);
        assert_eq!(Solution::min_side_jumps(vec![0, 1, 1, 3, 3, 0]), 0);
        assert_eq!(Solution::min_side_jumps(vec![0, 2, 1, 0, 3, 0]), 2);
    }
}
