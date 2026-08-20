// https://leetcode.com/problems/minimum-operations-to-write-the-letter-y-on-a-grid/
// 3071. Minimum Operations to Write the Letter Y
pub struct Solution;
impl Solution {
    pub fn minimum_operations_to_write_y(grid: Vec<Vec<i32>>) -> i32 {
        let mut cnt_y = [0, 0, 0];
        let mut cnt_ny = [0, 0, 0];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if i <= grid.len() / 2 {
                    if i == j || i + j == grid.len() - 1 {
                        cnt_y[grid[i][j] as usize] += 1;
                    } else {
                        cnt_ny[grid[i][j] as usize] += 1;
                    }
                } else if j == grid.len() / 2 {
                    cnt_y[grid[i][j] as usize] += 1;
                } else {
                    cnt_ny[grid[i][j] as usize] += 1;
                }
            }
        }
        let mut cnt_y_sort = cnt_y.into_iter().enumerate().collect::<Vec<_>>();
        cnt_y_sort.sort_by(|a, b| b.1.cmp(&a.1));
        let mut cnt_ny_sort = cnt_ny.into_iter().enumerate().collect::<Vec<_>>();
        cnt_ny_sort.sort_by(|a, b| b.1.cmp(&a.1));
        if cnt_y_sort[0].0 != cnt_ny_sort[0].0 {
            (grid.len() * grid.len()) as i32 - cnt_y_sort[0].1 - cnt_ny_sort[0].1
        } else {
            (grid.len() * grid.len()) as i32
                - (cnt_y_sort[0].1 + cnt_ny_sort[1].1).max(cnt_y_sort[1].1 + cnt_ny_sort[0].1)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn minimum_operations_to_write_y() {
        assert_eq!(
            Solution::minimum_operations_to_write_y(vec_vec![[1, 2, 2], [1, 1, 0], [0, 1, 0]]),
            3
        );
        assert_eq!(
            Solution::minimum_operations_to_write_y(vec_vec![
                [0, 1, 0, 1, 0],
                [2, 1, 0, 1, 2],
                [2, 2, 2, 0, 1],
                [2, 2, 2, 2, 2],
                [2, 1, 2, 2, 2]
            ]),
            12
        );
    }
}
