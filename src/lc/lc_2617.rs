// https://leetcode.com/problems/minimum-number-of-visited-cells-in-a-grid/
// 2617. Minimum Number of Visited Cells in a Grid
pub struct Solution;
impl Solution {
    pub fn minimum_visited_cells(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        if m == 1 && n == 1 {
            return 1;
        }
        let mut cols = vec![std::collections::BinaryHeap::new(); n];
        for i in 0..m {
            let mut row = std::collections::BinaryHeap::new();
            if i == 0 {
                cols[0].push(std::cmp::Reverse((1, 0)));
                row.push(std::cmp::Reverse((1, 0)));
            }
            for j in 0..n {
                if i == 0 && j == 0 {
                    continue;
                }
                let mut min_step = i32::MAX;
                while let Some(&std::cmp::Reverse((step, idx))) = cols[j].peek() {
                    if grid[idx][j] as usize + idx >= i {
                        min_step = step + 1;
                        break;
                    }
                    cols[j].pop();
                }
                while let Some(&std::cmp::Reverse((step, idx))) = row.peek() {
                    if grid[i][idx] as usize + idx >= j {
                        min_step = min_step.min(step + 1);
                        break;
                    }
                    row.pop();
                }
                if min_step != i32::MAX {
                    cols[j].push(std::cmp::Reverse((min_step, i)));
                    row.push(std::cmp::Reverse((min_step, j)));
                }
                if i == m - 1 && j == n - 1 {
                    return if min_step == i32::MAX { -1 } else { min_step };
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_minimum_visited_cells() {
        assert_eq!(Solution::minimum_visited_cells(vec_vec![[0]]), 1);
        assert_eq!(
            Solution::minimum_visited_cells(vec_vec![
                [7, 12, 11, 11, 4],
                [10, 5, 16, 15, 7],
                [7, 9, 6, 16, 7],
                [1, 13, 3, 16, 0]
            ]),
            3
        );
        assert_eq!(
            Solution::minimum_visited_cells(vec_vec![[3, 4, 2, 1], [4, 2, 3, 1], [2, 1, 0, 0], [2, 4, 0, 0]]),
            4
        );
        assert_eq!(
            Solution::minimum_visited_cells(vec_vec![[3, 4, 2, 1], [4, 2, 1, 1], [2, 1, 1, 0], [3, 4, 1, 0]]),
            3
        );
        assert_eq!(Solution::minimum_visited_cells(vec_vec![[2, 1, 0], [1, 0, 0]]), -1);
    }
}
