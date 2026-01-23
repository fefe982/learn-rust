// https://leetcode.com/problems/equal-sum-grid-partition-ii/
// 3548. Equal Sum Grid Partition II
pub struct Solution;
impl Solution {
    pub fn partition<F>(g: F, m: usize, n: usize, sum: i64) -> bool
    where
        F: Fn(usize, usize) -> i64,
    {
        if m == 1 {
            return false;
        }
        if n == 1 {
            let mut s = 0;
            for i in 0..m - 1 {
                s += g(i, 0) as i64;
                let d = s * 2 - sum;
                if d == 0 || d == g(0, 0) || d == g(i, 0) {
                    return true;
                }
            }
        } else {
            let mut s = 0;
            let mut set = std::collections::HashSet::new();
            for i in 0..m - 1 {
                for j in 0..n {
                    s += g(i, j);
                    set.insert(g(i, j));
                }
                let d = s * 2 - sum;
                if d == 0 {
                    return true;
                }
                if i == 0 {
                    if d == g(0, 0) || d == g(0, n - 1) {
                        return true;
                    }
                } else if set.contains(&d) {
                    return true;
                }
            }
        }
        false
    }
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let sum = grid.iter().flatten().map(|&i| i as i64).sum::<i64>();
        Self::partition(|i, j| grid[i][j] as i64, m, n, sum)
            || Self::partition(|i, j| grid[m - 1 - i][j] as i64, m, n, sum)
            || Self::partition(|i, j| grid[j][i] as i64, n, m, sum)
            || Self::partition(|i, j| grid[j][n - 1 - i] as i64, n, m, sum)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn can_partition_grid() {
        assert_eq!(Solution::can_partition_grid(vec_vec![[1, 4], [2, 3]]), true);
        assert_eq!(Solution::can_partition_grid(vec_vec![[1, 2], [3, 4]]), true);
        assert_eq!(Solution::can_partition_grid(vec_vec![[1, 2, 4], [2, 3, 5]]), false);
    }
}
