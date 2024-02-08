// https://leetcode.com/problems/minimum-moves-to-reach-target-with-rotations/
// 1210. Minimum Moves to Reach Target with Rotations
pub struct Solution;
impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut visited = vec![vec![vec![0; 2]; n]; n];
        visited[0][0][0] = 1;
        let mut q = std::collections::VecDeque::new();
        q.push_back((0, 0, 0, 0));
        while let Some((i, j, dir, step)) = q.pop_front() {
            // right
            if dir == 0 && j + 2 < n && grid[i][j + 2] == 0 && visited[i][j + 1][0] == 0 {
                if i == n - 1 && j == n - 3 {
                    return step + 1;
                }
                visited[i][j + 1][0] = 1;
                q.push_back((i, j + 1, 0, step + 1));
            }
            if dir == 1 && j + 1 < n && grid[i][j + 1] == 0 && grid[i + 1][j + 1] == 0 && visited[i][j + 1][1] == 0 {
                visited[i][j + 1][1] = 1;
                q.push_back((i, j + 1, 1, step + 1));
            }
            // down
            if dir == 0 && i + 1 < n && grid[i + 1][j] == 0 && grid[i + 1][j + 1] == 0 && visited[i + 1][j][0] == 0 {
                if i + 1 == n - 1 && j == n - 2 {
                    return step + 1;
                }
                visited[i + 1][j][0] = 1;
                q.push_back((i + 1, j, 0, step + 1));
            }
            if dir == 1 && i + 2 < n && grid[i + 2][j] == 0 && visited[i + 1][j][1] == 0 {
                visited[i + 1][j][1] = 1;
                q.push_back((i + 1, j, 1, step + 1));
            }
            // rotate clock wise
            if dir == 0 && i + 1 < n && grid[i + 1][j] == 0 && grid[i + 1][j + 1] == 0 && visited[i][j][1] == 0 {
                visited[i][j][1] = 1;
                q.push_back((i, j, 1, step + 1));
            }
            // rotate counter clock wise
            if dir == 1 && j + 1 < n && grid[i][j + 1] == 0 && grid[i + 1][j + 1] == 0 && visited[i][j][0] == 0 {
                visited[i][j][0] = 1;
                q.push_back((i, j, 0, step + 1));
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
    fn test_minimum_moves() {
        assert_eq!(
            Solution::minimum_moves(vec_vec![
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                [0, 0, 1, 0, 1, 1, 1, 0, 0, 0],
                [0, 0, 1, 0, 0, 0, 1, 0, 1, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 1, 0, 0, 0, 0, 1, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 1, 0, 0, 0]
            ]),
            19
        );
        assert_eq!(
            Solution::minimum_moves(vec_vec![
                [0, 0, 0, 0, 1, 1],
                [1, 1, 0, 0, 0, 1],
                [1, 1, 1, 0, 0, 1],
                [1, 1, 1, 0, 1, 1],
                [1, 1, 1, 0, 0, 1],
                [1, 1, 1, 0, 0, 0]
            ]),
            11
        );
        assert_eq!(
            Solution::minimum_moves(vec_vec![
                [0, 0, 0, 0, 0, 1],
                [1, 1, 0, 0, 1, 0],
                [0, 0, 0, 0, 1, 1],
                [0, 0, 1, 0, 1, 0],
                [0, 1, 1, 0, 0, 0],
                [0, 1, 1, 0, 0, 0]
            ]),
            11
        );
        assert_eq!(
            Solution::minimum_moves(vec_vec![
                [0, 0, 1, 1, 1, 1],
                [0, 0, 0, 0, 1, 1],
                [1, 1, 0, 0, 0, 1],
                [1, 1, 1, 0, 0, 1],
                [1, 1, 1, 0, 0, 1],
                [1, 1, 1, 0, 0, 0]
            ]),
            9
        );
    }
}
