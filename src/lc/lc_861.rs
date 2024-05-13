// https://leetcode.com/problems/score-after-flipping-matrix/
// 861. Score After Flipping Matrix
pub struct Solution;
impl Solution {
    fn get_num(v: Vec<i32>) -> i32 {
        let i = 1 - v[0];
        let mut res = 0;
        for n in v {
            res = (res << 1) | (n ^ i);
        }
        res
    }
    pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
        let l = grid[0].len() - 1;
        let n = grid.len() as i32;
        let mut sum = n * (1 << l);
        let grid = grid.into_iter().map(Solution::get_num).collect::<Vec<_>>();
        for i in 0..l {
            let mask = 1 << i;
            let mut cnt = 0;
            for &num in &grid {
                if num & mask != 0 {
                    cnt += 1;
                }
            }
            sum += cnt.max(n - cnt) * mask;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_matrix_score() {
        assert_eq!(
            Solution::matrix_score(vec_vec![[0, 0, 1, 1], [1, 0, 1, 0], [1, 1, 0, 0]]),
            39
        );
        assert_eq!(Solution::matrix_score(vec_vec![[0]]), 1);
    }
}
