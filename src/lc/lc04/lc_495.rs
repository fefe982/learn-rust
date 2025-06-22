// https://leetcode.com/problems/teemo-attacking/
// 495. Teemo Attacking
pub struct Solution;
impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        if time_series.is_empty() {
            return 0;
        }
        let mut total_duration = 0;
        for i in 1..time_series.len() {
            let diff = time_series[i] - time_series[i - 1];
            total_duration += diff.min(duration);
        }
        total_duration + duration
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_poisoned_duration() {
        assert_eq!(Solution::find_poisoned_duration(vec![1, 4], 2), 4);
        assert_eq!(Solution::find_poisoned_duration(vec![1, 2], 2), 3);
    }
}
