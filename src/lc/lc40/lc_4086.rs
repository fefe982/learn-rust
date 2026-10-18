// https://leetcode.com/problems/minimum-interval-selection-to-cover-target-intervals/
// 4086. Minimum Interval Selection to Cover Target Intervals
pub struct Solution;
impl Solution {
    pub fn min_selection(available_intervals: Vec<Vec<i32>>, target_intervals: Vec<Vec<i32>>) -> i32 {
        let mut available_intervals = available_intervals;
        available_intervals.sort();
        let mut target_intervals = target_intervals;
        target_intervals.sort();
        let mut ans = 0;
        let mut last = 0;
        let mut itarget = 0;
        let mut iavail = 0;
        let mut target = 0;
        loop {
            if itarget >= target_intervals.len() {
                return ans;
            }
            if iavail >= available_intervals.len() {
                return -1;
            }
            target = target.max(target_intervals[itarget][0]);
            let mut nlast = last;
            while iavail < available_intervals.len() && available_intervals[iavail][0] <= target {
                nlast = available_intervals[iavail][1].max(nlast);
                iavail += 1;
            }
            if nlast < target {
                return -1;
            }
            if nlast == last && nlast == target {
                return -1;
            }
            last = nlast;
            ans += 1;
            target = last;
            while itarget < target_intervals.len() && target_intervals[itarget][1] <= target {
                itarget += 1;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_selection() {
        assert_eq!(
            Solution::min_selection(
                vec_vec![[3, 29], [2, 2], [30, 30]],
                vec_vec![
                    [3, 9],
                    [2, 2],
                    [28, 28],
                    [7, 7],
                    [20, 25],
                    [5, 28],
                    [29, 29],
                    [16, 16],
                    [10, 10],
                    [16, 29],
                    [23, 23],
                    [20, 20],
                    [12, 12],
                    [11, 11],
                    [26, 26],
                    [30, 30],
                    [3, 3],
                    [13, 13],
                    [21, 21]
                ]
            ),
            3
        );
        assert_eq!(
            Solution::min_selection(
                vec_vec![[1, 4], [1, 7], [3, 9], [5, 10], [10, 15]],
                vec_vec![[1, 3], [2, 5], [7, 8], [10, 13], [11, 15], [12, 13]]
            ),
            3
        );
        assert_eq!(
            Solution::min_selection(vec_vec![[4, 6], [1, 4], [2, 5]], vec_vec![[3, 4], [2, 5], [6, 7]]),
            -1
        );
        assert_eq!(Solution::min_selection(vec_vec![[1, 2], [3, 4]], vec_vec![[1, 4]]), -1);
    }
}
