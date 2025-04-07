// https://leetcode.com/problems/find-right-interval/
// 436. Find Right Interval
pub struct Solution;
impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut idx = (0..intervals.len()).collect::<Vec<_>>();
        idx.sort_by_key(|&i| intervals[i][0]);
        let mut result = vec![-1; intervals.len()];
        for i in 0..intervals.len() {
            let p = idx.partition_point(|&j| intervals[j][0] < intervals[i][1]);
            if p < intervals.len() {
                result[i] = idx[p] as i32;
            }
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_right_interval() {
        assert_eq!(Solution::find_right_interval(vec_vec![[1, 2]]), vec![-1]);
        assert_eq!(
            Solution::find_right_interval(vec_vec![[3, 4], [2, 3], [1, 2]]),
            vec![-1, 0, 1]
        );
        assert_eq!(
            Solution::find_right_interval(vec_vec![[1, 4], [2, 3], [3, 4]]),
            vec![-1, 2, -1]
        );
    }
}
