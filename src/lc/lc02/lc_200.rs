// https://leetcode.com/problems/number-of-islands/
// 200. Number of Islands
pub struct Solution;
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut n = 0;
        let mut grid = grid;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    n += 1;
                    let mut q = vec![(i, j)];
                    grid[i][j] = '0';
                    while let Some((x, y)) = q.pop() {
                        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                            let nx = (x as i32 + dx) as usize;
                            let ny = (y as i32 + dy) as usize;
                            if nx < grid.len() && ny < grid[0].len() && grid[nx][ny] == '1' {
                                q.push((nx, ny));
                                grid[nx][ny] = '0';
                            }
                        }
                    }
                }
            }
        }
        n
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_num_islands() {
        assert_eq!(
            Solution::num_islands(vec_vec_chr![
                ["1", "1", "1", "1", "0"],
                ["1", "1", "0", "1", "0"],
                ["1", "1", "0", "0", "0"],
                ["0", "0", "0", "0", "0"]
            ]),
            1
        );
        assert_eq!(
            Solution::num_islands(vec_vec_chr![
                ["1", "1", "0", "0", "0"],
                ["1", "1", "0", "0", "0"],
                ["0", "0", "1", "0", "0"],
                ["0", "0", "0", "1", "1"]
            ]),
            3
        );
    }
}
