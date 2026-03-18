// https://leetcode.cn/problems/count-submatrices-with-equal-frequency-of-x-and-y/
// 3212. Count Submatrices With Equal Frequency of X and Y
pub struct Solution;
impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut ans = 0;
        let mut cx = vec![0; m + 1];
        let mut cy = vec![0; m + 1];
        for i in 0..n {
            let mut ccx = vec![0; m + 1];
            let mut ccy = vec![0; m + 1];
            let mut rx = 0;
            let mut ry = 0;
            for j in 0..m {
                if grid[i][j] == 'X' {
                    rx += 1;
                } else if grid[i][j] == 'Y' {
                    ry += 1;
                }
                ccx[j + 1] = rx + cx[j + 1];
                ccy[j + 1] = ry + cy[j + 1];
                if ccx[j + 1] == ccy[j + 1] && ccx[j + 1] != 0 {
                    ans += 1;
                }
            }
            cx = ccx;
            cy = ccy;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn number_of_submatrices() {
        assert_eq!(
            Solution::number_of_submatrices(vec_vec_chr![["X", "Y", "."], ["Y", ".", "."]]),
            3
        );
        assert_eq!(Solution::number_of_submatrices(vec_vec_chr![["X", "X"], ["X", "Y"]]), 0);
        assert_eq!(Solution::number_of_submatrices(vec_vec_chr![[".", "."], [".", "."]]), 0);
    }
}
