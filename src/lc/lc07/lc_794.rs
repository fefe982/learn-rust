// https://leetcode.com/problems/valid-tic-tac-toe-state/
// 794. Valid Tic-Tac-Toe State
pub struct Solution;
impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let mut cx = [[0, 0]; 4];
        let mut co = [[0, 0]; 4];
        let mut x = 0;
        let mut o = 0;
        for (i, s) in board.iter().enumerate() {
            for (j, c) in s.chars().enumerate() {
                match c {
                    'X' => {
                        cx[i][0] += 1;
                        cx[j][1] += 1;
                        if i == j {
                            cx[3][0] += 1;
                        }
                        if i + j == 2 {
                            cx[3][1] += 1;
                        }
                        x += 1;
                    }
                    'O' => {
                        co[i][0] += 1;
                        co[j][1] += 1;
                        if i == j {
                            co[3][0] += 1;
                        }
                        if i + j == 2 {
                            co[3][1] += 1;
                        }
                        o += 1;
                    }
                    _ => (),
                }
            }
        }
        if x < o || x - o > 1 {
            return false;
        }
        let wx = cx[0][0] == 3
            || cx[1][0] == 3
            || cx[2][0] == 3
            || cx[3][0] == 3
            || cx[0][1] == 3
            || cx[1][1] == 3
            || cx[2][1] == 3
            || cx[3][1] == 3;
        let wo = co[0][0] == 3
            || co[1][0] == 3
            || co[2][0] == 3
            || co[3][0] == 3
            || co[0][1] == 3
            || co[1][1] == 3
            || co[2][1] == 3
            || co[3][1] == 3;
        if wx && (wo || o == x) {
            return false;
        }
        if wo && x > o {
            return false;
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn valid_tic_tac_toe() {
        assert_eq!(Solution::valid_tic_tac_toe(vec_str!["O  ", "   ", "   "]), false);
        assert_eq!(Solution::valid_tic_tac_toe(vec_str!["XOX", " X ", "   "]), false);
        assert_eq!(Solution::valid_tic_tac_toe(vec_str!["XOX", "O O", "XOX"]), true);
    }
}
