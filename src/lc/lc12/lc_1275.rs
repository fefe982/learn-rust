// https://leetcode.com/problems/find-winner-on-a-tic-tac-toe-game/
// 1275. Find Winner on a Tic Tac Toe Game
pub struct Solution;
impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut board = vec![vec![0; 3]; 3];
        for (i, move_) in moves.iter().enumerate() {
            board[move_[0] as usize][move_[1] as usize] = if i % 2 == 0 { 1 } else { 2 };
        }
        for i in 0..3 {
            if board[i][0] == board[i][1] && board[i][1] == board[i][2] && board[i][0] != 0 {
                return if board[i][0] == 1 {
                    "A".to_string()
                } else {
                    "B".to_string()
                };
            }
            if board[0][i] == board[1][i] && board[1][i] == board[2][i] && board[0][i] != 0 {
                return if board[0][i] == 1 {
                    "A".to_string()
                } else {
                    "B".to_string()
                };
            }
        }
        if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] != 0 {
            return if board[0][0] == 1 {
                "A".to_string()
            } else {
                "B".to_string()
            };
        }
        if board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2] != 0 {
            return if board[0][2] == 1 {
                "A".to_string()
            } else {
                "B".to_string()
            };
        }
        if moves.len() == 9 {
            return "Draw".to_string();
        } else {
            return "Pending".to_string();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn tictactoe() {
        assert_eq!(
            Solution::tictactoe(vec_vec![[0, 0], [2, 0], [1, 1], [2, 1], [2, 2]]),
            "A"
        );
        assert_eq!(
            Solution::tictactoe(vec_vec![[0, 0], [1, 1], [0, 1], [0, 2], [1, 0], [2, 0]]),
            "B"
        );
        assert_eq!(
            Solution::tictactoe(vec_vec![
                [0, 0],
                [1, 1],
                [2, 0],
                [1, 0],
                [1, 2],
                [2, 1],
                [0, 1],
                [0, 2],
                [2, 2]
            ]),
            "Draw"
        );
    }
}
