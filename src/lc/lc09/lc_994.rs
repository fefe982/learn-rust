// https://leetcode.com/problems/rotting-oranges/
// 994. Rotting Oranges
use std::collections::VecDeque;
pub struct Solution;
impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut fresh = 0;
        let mut rotten = VecDeque::new();
        let xx = grid.len();
        let yy = grid[0].len();
        for (x, col) in grid.iter().enumerate() {
            for (y, &val) in col.iter().enumerate() {
                match val {
                    1 => {
                        fresh += 1;
                    }
                    2 => {
                        rotten.push_back((x, y));
                    }
                    _ => (),
                }
            }
        }
        if fresh == 0 {
            return 0;
        }
        if rotten.len() == 0 {
            return -1;
        }
        let mut last_val = 0;
        while let Some((x, y)) = rotten.pop_front() {
            last_val = grid[x][y];
            if x > 0 && grid[x - 1][y] == 1 {
                grid[x - 1][y] = last_val + 1;
                rotten.push_back((x - 1, y));
                fresh -= 1;
            }
            if x + 1 < xx && grid[x + 1][y] == 1 {
                grid[x + 1][y] = last_val + 1;
                rotten.push_back((x + 1, y));
                fresh -= 1;
            }
            if y > 0 && grid[x][y - 1] == 1 {
                grid[x][y - 1] = last_val + 1;
                rotten.push_back((x, y - 1));
                fresh -= 1;
            }
            if y + 1 < yy && grid[x][y + 1] == 1 {
                grid[x][y + 1] = last_val + 1;
                rotten.push_back((x, y + 1));
                fresh -= 1;
            }
        }
        if fresh > 0 {
            -1
        } else {
            last_val - 2
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oranges_rotting() {
        assert_eq!(
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]),
            4
        );
        assert_eq!(
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]),
            -1
        );
        assert_eq!(Solution::oranges_rotting(vec![vec![0, 2]]), 0);
    }
}
