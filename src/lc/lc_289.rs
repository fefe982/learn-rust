// https://leetcode.com/problems/game-of-life/
// 289. Game of Life
pub struct Solution;
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let tmp = board.clone();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                let mut cnt = 0;
                for (x, y) in vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
                    let nx = (i as i32 + x) as usize;
                    let ny = (j as i32 + y) as usize;
                    if nx < board.len() && ny < board[0].len() {
                        cnt += tmp[nx][ny];
                    }
                }
                if tmp[i][j] == 1 {
                    if cnt < 2 || cnt > 3 {
                        board[i][j] = 0;
                    }
                } else {
                    if cnt == 3 {
                        board[i][j] = 1;
                    }
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn check(board: Vec<Vec<i32>>, expect: Vec<Vec<i32>>) {
        let mut board = board;
        Solution::game_of_life(&mut board);
        assert_eq!(board, expect);
    }
    #[test]
    fn game_of_life() {
        check(
            vec_vec![[0, 1, 0], [0, 0, 1], [1, 1, 1], [0, 0, 0]],
            vec_vec![[0, 0, 0], [1, 0, 1], [0, 1, 1], [0, 1, 0]],
        );
        check(vec_vec![[1, 1], [1, 0]], vec_vec![[1, 1], [1, 1]]);
    }
}
