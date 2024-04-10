// https://leetcode.com/problems/find-minimum-time-to-finish-all-jobs/
// 1723. Find Minimum Time to Finish All Jobs
pub struct Solution;
impl Solution {
    fn get_workers(job_sum: &Vec<i32>, time: i32) -> i32 {
        let mut dp = vec![20; job_sum.len()];
        dp[0] = 0;
        for i in 1..job_sum.len() {
            if job_sum[i] <= time {
                dp[i] = 1;
                continue;
            }
            let mut j = i & (i - 1);
            while j > i - j {
                dp[i] = dp[i].min(dp[j] + dp[i - j]);
                j = i & (j - 1);
            }
        }
        dp[job_sum.len() - 1]
    }
    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        let n = jobs.len() as i32;
        let sz = 1 << n;
        let mut sum = vec![0; sz];
        for i in 1..sz {
            if i.is_power_of_two() {
                sum[i] = jobs[i.trailing_zeros() as usize];
            } else {
                sum[i] = sum[i & (i - 1)] + sum[i & !(i - 1)]
            }
        }
        let mut l = jobs[0];
        let mut r = sum[sz - 1];
        if Self::get_workers(&sum, l) <= k {
            return l;
        }
        while l + 1 < r {
            let mid = (l + r) / 2;
            if Self::get_workers(&sum, mid) <= k {
                r = mid;
            } else {
                l = mid;
            }
        }
        r
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_time_required() {
        assert_eq!(Solution::minimum_time_required(vec![3, 2, 3], 3), 3);
        assert_eq!(Solution::minimum_time_required(vec![1, 2, 4, 7, 8], 2), 11);
    }
}
