// https://leetcode.com/problems/nearest-exit-from-entrance-in-maze/
// 1926. Nearest Exit from Entrance in Maze
pub struct Solution;
impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let m = maze.len();
        let n = maze[0].len();
        let mut visited = vec![vec![false; n]; m];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((entrance[0] as usize, entrance[1] as usize, 0));
        visited[entrance[0] as usize][entrance[1] as usize] = true;
        while let Some((row, col, dist)) = queue.pop_front() {
            if (row == 0 || row == m - 1 || col == 0 || col == n - 1) && (row != entrance[0] as usize || col != entrance[1] as usize) {
                return dist;
            }
            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;
                if new_row >= 0
                    && new_row < m as i32
                    && new_col >= 0
                    && new_col < n as i32
                    && maze[new_row as usize][new_col as usize] == '.'
                    && !visited[new_row as usize][new_col as usize]
                {
                    visited[new_row as usize][new_col as usize] = true;
                    queue.push_back((new_row as usize, new_col as usize, dist + 1));
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
    fn nearest_exit() {
        assert_eq!(
            Solution::nearest_exit(
                vec_vec_chr![["+", "+", ".", "+"], [".", ".", ".", "+"], ["+", "+", "+", "."]],
                vec![1, 2]
            ),
            1
        );
        assert_eq!(
            Solution::nearest_exit(
                vec_vec_chr![["+", "+", "+"], [".", ".", "."], ["+", "+", "+"]],
                vec![1, 0]
            ),
            2
        );
        assert_eq!(Solution::nearest_exit(vec_vec_chr![[".", "+"]], vec![0, 0]), -1);
    }
}
