// https://leetcode.com/problems/check-if-there-is-a-valid-path-in-a-grid/
// 1391. Check If There Is a Valid Path in a Grid
pub struct Solution;
impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, 0, 0));
        queue.push_back((0, 0, 1));
        queue.push_back((0, 0, 2));
        queue.push_back((0, 0, 3));
        while !queue.is_empty() {
            let (y, x, d) = queue.pop_front().unwrap();
            let nx;
            let ny;
            let nd;
            match d {
                0 => match grid[y][x] {
                    1 => {
                        nx = x + 1;
                        ny = y;
                        nd = 0;
                    }
                    3 => {
                        nx = x;
                        ny = y + 1;
                        nd = 1;
                    }
                    5 => {
                        nx = x;
                        ny = y.wrapping_sub(1);
                        nd = 3;
                    }
                    _ => {
                        continue;
                    }
                },
                1 => match grid[y][x] {
                    2 => {
                        nx = x;
                        ny = y + 1;
                        nd = 1;
                    }
                    5 => {
                        nx = x.wrapping_sub(1);
                        ny = y;
                        nd = 2;
                    }
                    6 => {
                        nx = x + 1;
                        ny = y;
                        nd = 0;
                    }
                    _ => {
                        continue;
                    }
                },
                2 => match grid[y][x] {
                    1 => {
                        nx = x.wrapping_sub(1);
                        ny = y;
                        nd = 2;
                    }
                    4 => {
                        nx = x;
                        ny = y + 1;
                        nd = 1;
                    }
                    6 => {
                        nx = x;
                        ny = y.wrapping_sub(1);
                        nd = 3;
                    }
                    _ => {
                        continue;
                    }
                },
                3 => match grid[y][x] {
                    2 => {
                        nx = x;
                        ny = y.wrapping_sub(1);
                        nd = 3;
                    }
                    3 => {
                        nx = x.wrapping_sub(1);
                        ny = y;
                        nd = 2;
                    }
                    4 => {
                        nx = x + 1;
                        ny = y;
                        nd = 0;
                    }
                    _ => {
                        continue;
                    }
                },
                _ => {
                    unreachable!()
                }
            }
            visited[y][x] = true;
            if y == grid.len() - 1 && x == grid[0].len() - 1 {
                return true;
            }
            if ny < grid.len() && nx < grid[0].len() && !visited[ny][nx] {
                queue.push_back((ny, nx, nd));
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn has_valid_path() {
        assert_eq!(Solution::has_valid_path(vec_vec![[4, 1], [6, 1]]), true);
        assert_eq!(Solution::has_valid_path(vec_vec![[2, 4, 3], [6, 5, 2]]), true);
        assert_eq!(Solution::has_valid_path(vec_vec![[1, 2, 1], [1, 2, 1]]), false);
        assert_eq!(Solution::has_valid_path(vec_vec![[1, 1, 2]]), false);
    }
}
