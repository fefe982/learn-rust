// https://leetcode.com/problems/check-if-word-can-be-placed-in-crossword/
// 2018. Check if Word Can Be Placed in Crossword
pub struct Solution;
impl Solution {
    fn place_word(get: impl Fn(usize, usize) -> char, m: usize, n: usize, word: &[u8]) -> bool {
        let wn = word.len();
        for i in 0..m {
            let mut s = 0;
            while s < n {
                while s < n && get(i, s) == '#' {
                    s += 1;
                }
                if s >= n {
                    break;
                }
                let mut t = s;
                while t < n && get(i, t) != '#' {
                    t += 1;
                }
                if t - s == word.len() {
                    for j in 0..wn {
                        if get(i, s + j) != word[j] as char && get(i, s + j) != ' ' {
                            break;
                        }
                        if j == wn - 1 {
                            return true;
                        }
                    }
                }
                s = t;
            }
        }
        false
    }
    pub fn place_word_in_crossword(board: Vec<Vec<char>>, word: String) -> bool {
        let wb = word.as_bytes();
        let m = board.len();
        let n = board[0].len();
        Self::place_word(|i, j| board[i][j], m, n, wb)
            || Self::place_word(|i, j| board[i][n - j - 1], m, n, wb)
            || Self::place_word(|i, j| board[j][i], n, m, wb)
            || Self::place_word(|i, j| board[m - j - 1][i], n, m, wb)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn place_word_in_crossword() {
        assert_eq!(
            Solution::place_word_in_crossword(
                vec_vec_chr![["#", " ", "#"], [" ", " ", "#"], ["#", "c", " "]],
                "abc".to_string()
            ),
            true
        );
        assert_eq!(
            Solution::place_word_in_crossword(
                vec_vec_chr![[" ", "#", "a"], [" ", "#", "c"], [" ", "#", "a"]],
                "ac".to_string()
            ),
            false
        );
        assert_eq!(
            Solution::place_word_in_crossword(
                vec_vec_chr![["#", " ", "#"], [" ", " ", "#"], ["#", " ", "c"]],
                "ca".to_string()
            ),
            true
        );
    }
}
