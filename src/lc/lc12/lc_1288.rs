// https://leetcode.com/problems/remove-covered-intervals/
// 1288. Remove Covered Intervals
pub struct Solution;
impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by_key(|a| (a[0], -a[1]));
        let mut end = 0;
        let mut cnt = 0;
        for interval in intervals {
            if interval[1] > end {
                end = interval[1];
                cnt += 1;
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn remove_covered_intervals() {
        assert_eq!(Solution::remove_covered_intervals(vec_vec![[1, 4], [3, 6], [2, 8]]), 2);
        assert_eq!(Solution::remove_covered_intervals(vec_vec![[1, 4], [2, 3]]), 1);
    }
}
