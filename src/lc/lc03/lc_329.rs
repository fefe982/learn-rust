// https://leetcode.com/problems/longest-increasing-path-in-a-matrix/
// 329. Longest Increasing Path in a Matrix
pub struct Solution;
impl Solution {
    fn get_max_len(
        matrix: &Vec<Vec<i32>>,
        ih: usize,
        iw: usize,
        max_len: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if max_len[ih][iw] > 0 {
            return max_len[ih][iw];
        }
        let mut m = 0;
        let c = matrix[ih][iw];
        for (nh, nw) in [
            (ih.saturating_sub(1), iw),
            (ih, iw.saturating_sub(1)),
            (ih + 1, iw),
            (ih, iw + 1),
        ] {
            if nh < matrix.len() && nw < matrix[0].len() && matrix[nh][nw] < c {
                m = m.max(Self::get_max_len(matrix, nh, nw, max_len));
            }
        }
        max_len[ih][iw] = m + 1;
        m + 1
    }
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let lh = matrix.len();
        let lw = matrix[0].len();
        let mut max_len = vec![vec![0; lw]; lh];
        let mut m = 0;
        for ih in 0..lh {
            for iw in 0..lw {
                m = m.max(Self::get_max_len(&matrix, ih, iw, &mut max_len));
            }
        }
        m
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn longest_increasing_path() {
        assert_eq!(
            Solution::longest_increasing_path(vec_vec![[9, 9, 4], [6, 6, 8], [2, 1, 1]]),
            4
        );
        assert_eq!(
            Solution::longest_increasing_path(vec_vec![[3, 4, 5], [3, 2, 6], [2, 2, 1]]),
            4
        );
        assert_eq!(Solution::longest_increasing_path(vec_vec![[1]]), 1);
    }
}
