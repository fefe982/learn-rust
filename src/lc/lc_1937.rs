// https://leetcode.com/problems/maximum-number-of-points-with-cost/
// 1937. Maximum Number of Points with Cost
pub struct Solution;
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let mut score = points[0].iter().map(|&x| x as i64).collect::<Vec<_>>();
        for i in 1..points.len() {
            let mut nscore = vec![0; score.len()];
            let max = score
                .iter()
                .scan(i64::MIN + 1, |state, &x| {
                    *state = (*state - 1).max(x as i64);
                    Some(*state)
                })
                .collect::<Vec<_>>();
            let mut m = i64::MIN + 1;
            for j in (0..score.len()).rev() {
                m = (m - 1).max(score[j] as i64);
                nscore[j] = points[i][j] as i64 + m.max(max[j]);
            }
            score = nscore;
        }
        *score.iter().max().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_points() {
        assert_eq!(Solution::max_points(vec_vec![[1, 2, 3], [1, 5, 1], [3, 1, 1]]), 9);
        assert_eq!(Solution::max_points(vec_vec![[1, 5], [2, 3], [4, 2]]), 11);
    }
}
