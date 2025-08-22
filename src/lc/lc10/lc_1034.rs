// https://leetcode.com/problems/coloring-a-border/
// 1034. Coloring A Border
pub struct Solution;
impl Solution {
    pub fn color_border(grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        let mut v = vec![vec![0; grid[0].len()]; grid.len()];
        let mut grid = grid;
        let mut stack = vec![];
        let mut border = vec![];
        stack.push((row as usize, col as usize));
        v[row as usize][col as usize] = 1;
        while let Some((row, col)) = stack.pop() {
            let mut flag = false;
            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (nr, nc) = ((row as isize + dr) as usize, (col as isize + dc) as usize);
                if nr < grid.len() && nc < grid[0].len() && grid[nr][nc] == grid[row][col] {
                    if v[nr][nc] == 0 {
                        stack.push((nr, nc));
                        v[nr][nc] = 1;
                    }
                } else {
                    flag = true;
                }
                if flag {
                    border.push((row, col));
                }
            }
        }
        for (row, col) in border {
            grid[row][col] = color;
        }
        grid
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn color_border() {
        assert_eq!(
            Solution::color_border(vec_vec![[1, 1], [1, 2]], 0, 0, 3),
            vec_vec![[3, 3], [3, 2]]
        );
        assert_eq!(
            Solution::color_border(vec_vec![[1, 2, 2], [2, 3, 2]], 0, 1, 3),
            vec_vec![[1, 3, 3], [2, 3, 3]]
        );
        assert_eq!(
            Solution::color_border(vec_vec![[1, 1, 1], [1, 1, 1], [1, 1, 1]], 1, 1, 2),
            vec_vec![[2, 2, 2], [2, 1, 2], [2, 2, 2]]
        );
    }
}
