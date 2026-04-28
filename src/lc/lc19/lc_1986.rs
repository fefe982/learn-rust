// https://leetcode.com/problems/minimum-number-of-work-sessions-to-finish-the-tasks/
// 1986. Minimum Number of Work Sessions to Finish the Tasks
pub struct Solution;
impl Solution {
    pub fn min_sessions(tasks: Vec<i32>, session_time: i32) -> i32 {
        let n = tasks.len();
        let size = 1usize << n;
        let mut sums = vec![0i32; size];
        for mask in 1..size {
            let lsb = mask & (!mask + 1);
            let idx = lsb.trailing_zeros() as usize;
            sums[mask] = sums[mask ^ lsb] + tasks[idx];
        }

        let mut dp = vec![i32::MAX / 2; size];
        dp[0] = 0;
        for mask in 1..size {
            let mut sub = mask;
            while sub > 0 {
                if sums[sub] <= session_time {
                    dp[mask] = dp[mask].min(dp[mask ^ sub] + 1);
                }
                sub = (sub - 1) & mask;
            }
        }
        dp[size - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_sessions() {
        assert_eq!(Solution::min_sessions(vec![1, 2, 3], 3), 2);
        assert_eq!(Solution::min_sessions(vec![3, 1, 3, 1, 1], 8), 2);
        assert_eq!(Solution::min_sessions(vec![1, 2, 3], 6), 1);
    }
}
