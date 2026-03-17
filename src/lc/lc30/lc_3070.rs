// https://leetcode.com/problems/count-submatrices-with-top-left-element-and-sum-less-than-k/
// 3070. Count Submatrices With Top-Left Element and Sum Less Than K
pub struct Solution;
impl Solution {
    pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut grid = grid;
        let m = grid.len();
        let n = grid[0].len();
        let mut r = 0;
        if grid[0][0] > k {
            return 0;
        }
        for j in 1..n {
            grid[0][j] += grid[0][j - 1];
            if grid[0][j] <= k {
                r = j;
            } else {
                break;
            }
        }
        let mut ans = r as i32 + 1;
        for i in 1..m {
            let mut sum = grid[i][0];
            grid[i][0] += grid[i - 1][0];
            if grid[i][0] > k {
                break;
            }
            r = 0;
            for j in 1..n {
                sum += grid[i][j];
                grid[i][j] = grid[i - 1][j] + sum;
                if grid[i][j] <= k {
                    r = j;
                } else {
                    break;
                }
            }
            ans += r as i32 + 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_submatrices() {
        assert_eq!(
            Solution::count_submatrices(vec_vec![[4, 8, 3], [9, 5, 1], [6, 8, 1]], 18),
            4
        );
        assert_eq!(Solution::count_submatrices(vec_vec![[7, 6, 3], [6, 6, 1]], 18), 4);
        assert_eq!(
            Solution::count_submatrices(vec_vec![[7, 2, 9], [1, 5, 0], [2, 6, 6]], 20),
            6
        );
    }
}
