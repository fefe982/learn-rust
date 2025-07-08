// https://leetcode.com/problems/reschedule-meetings-for-maximum-free-time-i/
// 3439. Reschedule Meetings for Maximum Free Time
pub struct Solution;
impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let mut start_time = start_time;
        let n = start_time.len();
        for i in 1..n {
            start_time[i] = start_time[i - 1] + start_time[i] - end_time[i - 1];
        }
        start_time.push(start_time[n - 1] + event_time - end_time[n - 1]);
        let k = k as usize;
        let mut res = start_time[k];
        for i in k + 1..start_time.len() {
            res = res.max(start_time[i] - start_time[i - k - 1]);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_free_time() {
        assert_eq!(Solution::max_free_time(5, 1, vec![1, 3], vec![2, 5]), 2);
        assert_eq!(Solution::max_free_time(5, 1, vec![0, 2, 9], vec![1, 4, 10]), 6);
        assert_eq!(
            Solution::max_free_time(5, 1, vec![0, 1, 2, 3, 4], vec![1, 2, 3, 4, 5]),
            0
        );
    }
}
