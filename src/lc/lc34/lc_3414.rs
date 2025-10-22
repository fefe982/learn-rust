// https://leetcode.com/problems/maximum-score-of-non-overlapping-intervals/
// 3414. Maximum Score of Non-Overlapping Intervals
pub struct Solution;
impl Solution {
    pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut intervals = intervals
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v[1], v[0], v[2], i as i32))
            .collect::<Vec<_>>();
        intervals.sort();
        let mut f = vec![vec![(0, vec![]); 5]; intervals.len() + 1];
        for i in 1..=intervals.len() {
            let (_, l, w, id) = intervals[i - 1];
            let pos = intervals[0..i].partition_point(|&(r, _, _, _)| r < l);
            for j in 1..5 {
                let (mut s, mut idx) = f[pos][j - 1].clone();
                idx.push(id);
                idx.sort();
                s += w as i64;
                if s > f[i - 1][j].0 || (s == f[i - 1][j].0 && idx < f[i - 1][j].1) {
                    f[i][j] = (s, idx);
                } else {
                    f[i][j] = f[i - 1][j].clone();
                }
            }
        }
        f[intervals.len()][4].1.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn maximum_weight() {
        assert_eq!(
            Solution::maximum_weight(vec_vec![
                [1, 1, 1000000000],
                [2, 2, 1000000000],
                [3, 3, 1000000000],
                [4, 4, 1000000000]
            ]),
            [0, 1, 2, 3]
        );
        assert_eq!(
            Solution::maximum_weight(vec_vec![
                [1, 3, 2],
                [4, 5, 2],
                [1, 5, 5],
                [6, 9, 3],
                [6, 7, 1],
                [8, 9, 1]
            ]),
            [2, 3]
        );
        assert_eq!(
            Solution::maximum_weight(vec_vec![
                [5, 8, 1],
                [6, 7, 7],
                [4, 7, 3],
                [9, 10, 6],
                [7, 8, 2],
                [11, 14, 3],
                [3, 5, 5]
            ]),
            [1, 3, 5, 6]
        );
    }
}
