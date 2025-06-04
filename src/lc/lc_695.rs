// https://leetcode.com/problems/max-area-of-island/
// 695. Max Area of Island
pub struct Solution;
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut max_area = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 1 {
                    let mut area = 1;
                    grid[i][j] = 0;
                    let mut stack = vec![(i, j)];
                    while let Some((x, y)) = stack.pop() {
                        for (i, j) in [(x + 1, y), (x.wrapping_sub(1), y), (x, y + 1), (x, y.wrapping_sub(1))] {
                            if i < grid.len() && j < grid[i].len() && grid[i][j] == 1 {
                                area += 1;
                                grid[i][j] = 0;
                                stack.push((i, j));
                            }
                        }
                    }
                    max_area = max_area.max(area);
                }
            }
        }
        max_area
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_area_of_island() {
        assert_eq!(
            Solution::max_area_of_island(vec_vec![
                [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                [0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                [0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
            ]),
            6
        );
        assert_eq!(Solution::max_area_of_island(vec_vec![[0, 0, 0, 0, 0, 0, 0, 0]]), 0);
    }
}
