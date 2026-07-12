// https://leetcode.com/problems/maximum-consistent-columns-in-a-grid/
// 3989. Maximum Number of Consecutive Ones
pub struct Solution;
impl Solution {
    pub fn max_consistent_columns(grid: Vec<Vec<i32>>, limit: i32) -> i32 {
        let mut m = vec![1; grid[0].len()];
        let mut max = 1;
        for i in 1..grid[0].len() {
            for j in 0..i {
                let mut good = true;
                for k in 0..grid.len() {
                    if (grid[k][i] - grid[k][j]).abs() > limit {
                        good = false;
                        break;
                    }
                }
                if good {
                    m[i] = m[i].max(m[j] + 1);
                }
            }
            max = max.max(m[i]);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_consistent_columns() {
        assert_eq!(Solution::max_consistent_columns(vec_vec![[-2, 0, 3]], 2), 2);
        assert_eq!(Solution::max_consistent_columns(vec_vec![[1, -1, 1], [2, 2, 2]], 1), 2);
        assert_eq!(Solution::max_consistent_columns(vec_vec![[-5, 5]], 9), 1);
    }
}
