// https://leetcode.com/problems/set-intersection-size-at-least-two/
// 757. Set Intersection Size At Least Two
pub struct Solution;
impl Solution {
    pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by_key(|a| (a[1], -a[0]));
        let mut ans = 2;
        let mut last = (intervals[0][1] - 1, intervals[0][1]);
        for i in 1..intervals.len() {
            if intervals[i][0] == last.1 {
                last = (last.1, intervals[i][1]);
                ans += 1;
            } else if intervals[i][0] > last.1 {
                ans += 2;
                last = (intervals[i][1] - 1, intervals[i][1]);
            } else if intervals[i][0] > last.0 {
                ans += 1;
                last = (last.1, intervals[i][1]);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_intersection_size_two() {
        assert_eq!(Solution::intersection_size_two(vec_vec![[1, 3], [3, 7], [8, 9]]), 5);
        assert_eq!(
            Solution::intersection_size_two(vec_vec![[1, 3], [1, 4], [2, 5], [3, 5]]),
            3
        );
        assert_eq!(
            Solution::intersection_size_two(vec_vec![[1, 2], [2, 3], [2, 4], [4, 5]]),
            5
        );
    }
}
