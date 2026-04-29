// https://leetcode.com/problems/maximum-path-score-in-a-grid/
// 3742. Maximum Path Score in a Grid
pub struct Solution;
impl Solution {
    pub fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let limit = k as usize;
        let mut prev = vec![vec![-1; limit + 1]; cols];

        for i in 0..rows {
            let mut curr = vec![vec![-1; limit + 1]; cols];
            for j in 0..cols {
                let score = grid[i][j];
                let cost = usize::from(score != 0);
                for used in cost..=limit {
                    let best_prev = if i == 0 && j == 0 {
                        0
                    } else {
                        let from_top = if i > 0 { prev[j][used - cost] } else { -1 };
                        let from_left = if j > 0 { curr[j - 1][used - cost] } else { -1 };
                        from_top.max(from_left)
                    };
                    if best_prev >= 0 {
                        curr[j][used] = best_prev + score;
                    }
                }
            }
            prev = curr;
        }

        *prev[cols - 1].iter().max().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_maximum_path_score_in_a_grid() {
        assert_eq!(Solution::max_path_score(vec_vec![[0, 1], [2, 0]], 1), 2);
        assert_eq!(Solution::max_path_score(vec_vec![[0, 1], [1, 2]], 1), -1);
        assert_eq!(Solution::max_path_score(vec_vec![[0, 2, 0]], 1), 2);
        assert_eq!(Solution::max_path_score(vec_vec![[0]], 0), 0);
    }
}
