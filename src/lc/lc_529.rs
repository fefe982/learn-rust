// https://leetcode.com/problems/minesweeper/
// 529. Minesweeper
pub struct Solution;
impl Solution {
    pub fn update_board(board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        let mut board = board;
        let (x, y) = (click[0] as usize, click[1] as usize);
        if board[x][y] == 'M' {
            board[x][y] = 'X';
        } else {
            let mut q = std::collections::VecDeque::new();
            q.push_back((x, y));
            while let Some((x, y)) = q.pop_front() {
                if board[x][y] != 'E' {
                    continue;
                }
                let mut cnt = 0;
                for (i, j) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
                    let nx = (x as i32 + i) as usize;
                    let ny = (y as i32 + j) as usize;
                    if nx < board.len() && ny < board[0].len() && board[nx][ny] == 'M' {
                        cnt += 1;
                    }
                }
                if cnt == 0 {
                    board[x][y] = 'B';
                    for (i, j) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
                        let nx = (x as i32 + i) as usize;
                        let ny = (y as i32 + j) as usize;
                        if nx < board.len() && ny < board[0].len() && board[nx][ny] == 'E' {
                            q.push_back((nx, ny));
                        }
                    }
                } else {
                    board[x][y] = (cnt as u8 + b'0') as char;
                }
            }
        }
        board
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn update_board() {
        assert_eq!(
            Solution::update_board(
                vec_vec_chr![
                    ["E", "E", "E", "E", "E"],
                    ["E", "E", "M", "E", "E"],
                    ["E", "E", "E", "E", "E"],
                    ["E", "E", "E", "E", "E"]
                ],
                vec![3, 0]
            ),
            vec_vec_chr![
                ["B", "1", "E", "1", "B"],
                ["B", "1", "M", "1", "B"],
                ["B", "1", "1", "1", "B"],
                ["B", "B", "B", "B", "B"]
            ]
        );
    }
}
