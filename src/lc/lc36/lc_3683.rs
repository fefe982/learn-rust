// https://leetcode.com/problems/earliest-time-to-finish-one-task/
// 3683. Earliest Time to Finish One Task
pub struct Solution;
impl Solution {
    pub fn earliest_time(tasks: Vec<Vec<i32>>) -> i32 {
        tasks.into_iter().map(|t| t[0] + t[1]).min().unwrap_or(0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn earliest_time() {
        assert_eq!(Solution::earliest_time(vec_vec![[1, 6], [2, 3]]), 5);
        assert_eq!(
            Solution::earliest_time(vec_vec![[100, 100], [100, 100], [100, 100]]),
            200
        );
    }
}
