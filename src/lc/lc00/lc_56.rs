// https://leetcode.com/problems/merge-intervals/
// 56. Merge Intervals
pub struct Solution;
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_unstable();
        let mut res = Vec::<Vec<i32>>::new();
        for i in intervals {
            let mut s = i[0];
            let mut e = i[1];
            if let Some(lv) = res.last() {
                if lv[1] >= s {
                    s = lv[0];
                    e = e.max(lv[1]);
                    res.pop();
                }
            }
            res.push(vec![s, e])
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn merge() {
        assert_eq!(
            Solution::merge(vec_vec![[1, 3], [2, 6], [8, 10], [15, 18]]),
            vec_vec![[1, 6], [8, 10], [15, 18]]
        );
        assert_eq!(Solution::merge(vec_vec![[1, 4], [4, 5]]), vec_vec![[1, 5]]);
    }
}
