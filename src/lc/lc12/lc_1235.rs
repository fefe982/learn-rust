// https://leetcode.com/problems/maximum-profit-in-job-scheduling/
// 1235. Maximum Profit in Job Scheduling
pub struct Solution;
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs = start_time
            .into_iter()
            .zip(end_time)
            .zip(profit)
            .map(|((s, e), p)| (s, e, p))
            .collect::<Vec<_>>();
        jobs.sort_unstable();
        let mut m = std::collections::BTreeMap::<i32, i32>::new();
        let mut max = 0;
        for (s, e, mut p) in jobs {
            if let Some((_, &v)) = m.range(..=s).rev().next() {
                p += v;
            }
            max = std::cmp::max(max, p);
            if let Some((_, &v)) = m.range(..=e).rev().next() {
                if p <= v {
                    continue;
                }
            }
            let mut r = vec![];
            for (&ke, &ve) in m.range(e..) {
                if ve <= p {
                    r.push(ke);
                }
            }
            for ke in r {
                m.remove(&ke);
            }
            m.insert(e, p);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_job_scheduling() {
        assert_eq!(
            Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]),
            120
        );
        assert_eq!(
            Solution::job_scheduling(vec![1, 2, 3, 4, 6], vec![3, 5, 10, 6, 9], vec![20, 20, 100, 70, 60]),
            150
        );
        assert_eq!(Solution::job_scheduling(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]), 6);
    }
}
