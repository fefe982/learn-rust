// https://leetcode.cn/problems/minimum-absolute-difference-in-sliding-submatrix/、
// 3567. Minimum Absolute Difference in Sliding Submatrix
pub struct Solution;
impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let k = k as usize;
        let mut ans = vec![vec![0; n - k + 1]; m - k + 1];
        for i in 0..m - k + 1 {
            for j in 0..n - k + 1 {
                let mut v = vec![];
                for x in i..i + k {
                    for y in j..j + k {
                        v.push(grid[x][y]);
                    }
                }
                v.sort();
                let mut min = i32::MAX;
                for x in 1..v.len() {
                    if v[x] != v[x - 1] {
                        min = min.min(v[x] - v[x - 1]);
                    }
                }
                if min != i32::MAX {
                    ans[i][j] = min;
                }
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
    fn min_abs_diff() {
        assert_eq!(Solution::min_abs_diff(vec_vec![[1, 8], [3, -2]], 2), vec_vec![[2]]);
        assert_eq!(Solution::min_abs_diff(vec_vec![[3, -1]], 1), vec_vec![[0, 0]]);
        assert_eq!(
            Solution::min_abs_diff(vec_vec![[1, -2, 3], [2, 3, 5]], 2),
            vec_vec![[1, 2]]
        );
    }
}
