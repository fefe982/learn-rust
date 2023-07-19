// https://leetcode.com/problems/non-overlapping-intervals/
// 435. Non-overlapping Intervals
pub struct Solution;
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        let mut ends = Vec::<(i32, i32)>::new();
        intervals.sort_unstable();
        let len = intervals.len();
        let mut last_len = 0;
        for interval in intervals {
            while let Some((end, len)) = ends.pop() {
                if end <= interval[0] {
                    last_len = len;
                } else {
                    ends.push((end.min(interval[1]), len));
                    break;
                }
            }
            if ends.is_empty() {
                ends.push((interval[1], last_len + 1));
            }
        }
        len as i32 - ends[0].1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn erase_overlap_intervals() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec_vec![[1, 2], [2, 3], [3, 4], [1, 3]]),
            1
        );
        assert_eq!(
            Solution::erase_overlap_intervals(vec_vec![[1, 2], [1, 2], [1, 2]]),
            2
        );
        assert_eq!(
            Solution::erase_overlap_intervals(vec_vec![[1, 2], [2, 3]]),
            0
        );
    }
}
