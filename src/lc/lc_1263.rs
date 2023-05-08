// https://leetcode.com/problems/minimum-moves-to-move-a-box-to-their-target-location/
// 1263. Minimum Moves to Move a Box to Their Target Location
pub struct Solution;
use std::collections::VecDeque;
const DIR: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
impl Solution {
    fn mv(p: (usize, usize), a: (i32, i32)) -> (usize, usize) {
        ((p.0 as i32 + a.0) as usize, (p.1 as i32 + a.1) as usize)
    }
    fn fill_stack(
        grid: &Vec<Vec<char>>,
        b: (usize, usize),
        w: (usize, usize),
        min: &mut Vec<Vec<Vec<Vec<i32>>>>,
        que: &mut VecDeque<((usize, usize), (usize, usize))>,
        t: (usize, usize),
    ) -> i32 {
        let mut stk: Vec<(usize, usize)> = vec![w];
        let cnt = min[b.0][b.1][w.0][w.1];
        while let Some(cw) = stk.pop() {
            for idx in 0..4 {
                let nw = Self::mv(cw, DIR[idx]);
                if nw.0 >= grid.len() || nw.1 >= grid[0].len() {
                    continue;
                }
                if grid[nw.0][nw.1] == '#' {
                    continue;
                }
                if nw == b {
                    let nb = Self::mv(b, DIR[idx]);
                    if nb.0 >= grid.len() || nb.1 >= grid[0].len() {
                        continue;
                    }
                    if nb == t {
                        return cnt + 1;
                    }
                    if grid[nb.0][nb.1] == '.' && min[nb.0][nb.1][nw.0][nw.1] < 0 {
                        min[nb.0][nb.1][nw.0][nw.1] = cnt + 1;
                        que.push_back((nb, nw));
                    }
                } else {
                    if min[b.0][b.1][nw.0][nw.1] < 0 {
                        min[b.0][b.1][nw.0][nw.1] = cnt;
                        stk.push(nw);
                    }
                }
            }
        }
        -1
    }
    pub fn min_push_box(mut grid: Vec<Vec<char>>) -> i32 {
        let mut t = (0, 0);
        let mut b = (0, 0);
        let mut w = (0, 0);
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                match grid[i][j] {
                    'T' => {
                        grid[i][j] = '.';
                        t = (i, j);
                    }
                    'S' => {
                        grid[i][j] = '.';
                        w = (i, j);
                    }
                    'B' => {
                        grid[i][j] = '.';
                        b = (i, j);
                    }
                    _ => (),
                }
            }
        }
        let mut min =
            vec![vec![vec![vec![-1; grid[0].len()]; grid.len()]; grid[0].len()]; grid.len()];
        min[b.0][b.1][w.0][w.1] = 0;
        let mut que: VecDeque<((usize, usize), (usize, usize))> = VecDeque::new();
        que.push_back((b, w));
        while let Some(state) = que.pop_front() {
            let cnt = Self::fill_stack(&grid, state.0, state.1, &mut min, &mut que, t);
            if cnt >= 0 {
                return cnt;
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
    fn min_push_box() {
        assert_eq!(
            Solution::min_push_box(vec_vec_chr![
                ["#", ".", ".", "#", "#", "#", "#", "#"],
                ["#", ".", ".", "T", "#", ".", ".", "#"],
                ["#", ".", ".", ".", "#", "B", ".", "#"],
                ["#", ".", ".", ".", ".", ".", ".", "#"],
                ["#", ".", ".", ".", "#", ".", "S", "#"],
                ["#", ".", ".", "#", "#", "#", "#", "#"]
            ]),
            7
        );
        assert_eq!(
            Solution::min_push_box(vec_vec_chr![
                ["#", "#", "#", "#", "#", "#"],
                ["#", "T", "#", "#", "#", "#"],
                ["#", ".", ".", "B", ".", "#"],
                ["#", ".", "#", "#", ".", "#"],
                ["#", ".", ".", ".", "S", "#"],
                ["#", "#", "#", "#", "#", "#"]
            ]),
            3
        );
        assert_eq!(
            Solution::min_push_box(vec_vec_chr![
                ["#", "#", "#", "#", "#", "#"],
                ["#", "T", "#", "#", "#", "#"],
                ["#", ".", ".", "B", ".", "#"],
                ["#", "#", "#", "#", ".", "#"],
                ["#", ".", ".", ".", "S", "#"],
                ["#", "#", "#", "#", "#", "#"]
            ]),
            -1
        );
        assert_eq!(
            Solution::min_push_box(vec_vec_chr![
                ["#", "#", "#", "#", "#", "#"],
                ["#", "T", ".", ".", "#", "#"],
                ["#", ".", "#", "B", ".", "#"],
                ["#", ".", ".", ".", ".", "#"],
                ["#", ".", ".", ".", "S", "#"],
                ["#", "#", "#", "#", "#", "#"]
            ]),
            5
        );
    }
}
