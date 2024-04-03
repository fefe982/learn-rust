// https://leetcode.com/problems/word-search/
// 79. Word Search
pub struct Solution;
impl Solution {
    fn search(
        board: &Vec<Vec<char>>,
        word: &Vec<char>,
        visited: &mut Vec<Vec<bool>>,
        x: usize,
        y: usize,
        i: usize,
    ) -> bool {
        if board[x][y] != word[i] {
            return false;
        }
        visited[x][y] = true;
        if i == word.len() - 1 {
            return true;
        }
        for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if nx < board.len() && ny < board[0].len() {
                if !visited[nx][ny] && Self::search(board, word, visited, nx, ny, i + 1) {
                    return true;
                }
            }
        }
        visited[x][y] = false;
        return false;
    }
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let m = board.len();
        let n = board[0].len();
        let word = word.chars().collect::<Vec<_>>();
        if word.len() > m * n {
            return false;
        }
        let mut freq = vec![0; 128];
        for i in 0..m {
            for j in 0..n {
                freq[board[i][j] as usize] += 1;
            }
        }
        for &c in &word {
            freq[c as usize] -= 1;
            if freq[c as usize] < 0 {
                return false;
            }
        }
        let mut visited = vec![vec![false; n]; m];
        for i in 0..m {
            for j in 0..n {
                if Self::search(&board, &word, &mut visited, i, j, 0) {
                    return true;
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
    fn test_exist() {
        assert_eq!(
            Solution::exist(
                vec_vec_chr![["A", "B", "C", "E"], ["S", "F", "C", "S"], ["A", "D", "E", "E"]],
                "ABCCED".to_string()
            ),
            true
        );
        assert_eq!(
            Solution::exist(
                vec_vec_chr![["A", "B", "C", "E"], ["S", "F", "C", "S"], ["A", "D", "E", "E"]],
                "SEE".to_string()
            ),
            true
        );
        assert_eq!(
            Solution::exist(
                vec_vec_chr![["A", "B", "C", "E"], ["S", "F", "C", "S"], ["A", "D", "E", "E"]],
                "ABCB".to_string()
            ),
            false
        );
    }
}
