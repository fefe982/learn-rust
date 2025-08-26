// https://leetcode.com/problems/length-of-longest-v-shaped-diagonal-segment/
// 3459. Length of Longest V-Shaped Diagonal Segment
pub struct Solution;
impl Solution {
    fn dfs(
        grid: &Vec<Vec<i32>>,
        i: usize,
        j: usize,
        dir: usize,
        turn: usize,
        expect: i32,
        s: &mut Vec<Vec<Vec<Vec<i32>>>>,
    ) -> i32 {
        if s[i][j][dir][turn] != -1 {
            return s[i][j][dir][turn];
        }
        let (di, dj) = match dir {
            0 => (1, 1),
            1 => (1, usize::MAX),
            2 => (usize::MAX, usize::MAX),
            _ => (usize::MAX, 1),
        };
        let ni = i.wrapping_add(di);
        let nj = j.wrapping_add(dj);
        if ni >= grid.len() || nj >= grid[0].len() || grid[ni][nj] != expect {
            return 0;
        }
        let mut res = Self::dfs(grid, ni, nj, dir, turn, 2 - expect, s);
        if turn == 0 {
            res = res.max(Self::dfs(grid, ni, nj, (dir + 1) % 4, 1, 2 - expect, s))
        }
        res += 1;
        s[i][j][dir][turn] = res;
        res
    }
    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let mut s = vec![vec![vec![vec![-1; 2]; 4]; grid[0].len()]; grid.len()];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    for dir in 0..4 {
                        max = max.max(1 + Self::dfs(&grid, i, j, dir, 0, 2, &mut s));
                    }
                }
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn len_of_v_diagonal() {
        assert_eq!(
            Solution::len_of_v_diagonal(vec_vec![[1, 1, 1, 2, 0, 0], [0, 0, 0, 0, 1, 2]]),
            2
        );
        assert_eq!(Solution::len_of_v_diagonal(vec_vec![[0, 0, 1, 0], [0, 2, 2, 0]]), 3);
        assert_eq!(
            Solution::len_of_v_diagonal(vec_vec![
                [2, 2, 1, 2, 2],
                [2, 0, 2, 2, 0],
                [2, 0, 1, 1, 0],
                [1, 0, 2, 2, 2],
                [2, 0, 0, 2, 2]
            ]),
            5
        );
        assert_eq!(
            Solution::len_of_v_diagonal(vec_vec![
                [2, 2, 2, 2, 2],
                [2, 0, 2, 2, 0],
                [2, 0, 1, 1, 0],
                [1, 0, 2, 2, 2],
                [2, 0, 0, 2, 2]
            ]),
            4
        );
        assert_eq!(
            Solution::len_of_v_diagonal(vec_vec![
                [1, 2, 2, 2, 2],
                [2, 2, 2, 2, 0],
                [2, 0, 0, 0, 0],
                [0, 0, 2, 2, 2],
                [2, 0, 0, 2, 0]
            ]),
            5
        );
        assert_eq!(Solution::len_of_v_diagonal(vec_vec![[1]]), 1);
    }
}
