// https://leetcode.com/problems/insert-interval/
// 57. Insert Interval
pub struct Solution;
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut i = 0;
        let mut res = vec![];
        while i < intervals.len() && intervals[i][1] < new_interval[0] {
            res.push(intervals[i].clone());
            i += 1;
        }
        let mut ni0 = new_interval[0];
        let mut ni1 = new_interval[1];
        if i < intervals.len() {
            ni0 = intervals[i][0].min(new_interval[0]);
        }
        while i < intervals.len() && intervals[i][0] <= new_interval[1] {
            ni1 = intervals[i][1].max(new_interval[1]);
            i += 1;
        }
        res.push(vec![ni0, ni1]);
        while i < intervals.len() {
            res.push(intervals[i].clone());
            i += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn insert() {
        assert_eq!(
            Solution::insert(vec_vec![[1, 3], [6, 9]], vec![2, 5]),
            vec_vec![[1, 5], [6, 9]]
        );
        assert_eq!(
            Solution::insert(vec_vec![[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]], vec![4, 8]),
            vec_vec![[1, 2], [3, 10], [12, 16]]
        );
    }
}
