// https://leetcode.com/problems/detect-cycles-in-2d-grid/
// 1559. Detect Cycles in 2D Grid
pub struct Solution;
impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if visited[i][j] {
                    continue;
                }
                let mut q = std::collections::VecDeque::new();
                q.push_back((i, j, i, j));
                visited[i][j] = true;
                while let Some((x, y, px, py)) = q.pop_front() {
                    for (nx, ny) in [(x + 1, y), (x.wrapping_sub(1), y), (x, y + 1), (x, y.wrapping_sub(1))] {
                        if nx < grid.len() && ny < grid[0].len() && grid[nx][ny] == grid[x][y] {
                            if nx == px && ny == py {
                                continue;
                            }
                            if visited[nx][ny] {
                                return true;
                            }
                            visited[nx][ny] = true;
                            q.push_back((nx, ny, x, y));
                        }
                    }
                }
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
    fn contains_cycle() {
        assert_eq!(
            Solution::contains_cycle(vec_vec_chr![
                ["a", "a", "a", "a"],
                ["a", "b", "b", "a"],
                ["a", "b", "b", "a"],
                ["a", "a", "a", "a"]
            ]),
            true
        );
        assert_eq!(
            Solution::contains_cycle(vec_vec_chr![
                ["c", "c", "c", "a"],
                ["c", "d", "c", "c"],
                ["c", "c", "e", "c"],
                ["f", "c", "c", "c"]
            ]),
            true
        );
        assert_eq!(
            Solution::contains_cycle(vec_vec_chr![["a", "b", "b"], ["b", "z", "b"], ["b", "b", "a"]]),
            false
        );
    }
}
