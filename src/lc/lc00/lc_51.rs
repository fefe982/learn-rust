// https://leetcode.com/problems/n-queens/
// 51. N-Queens
pub struct Solution;
impl Solution {
    fn conv(s: &Vec<i32>) -> Vec<String> {
        let n = s.len();
        let mut res = Vec::with_capacity(n);
        let mut line = vec![b'.'; n];
        for &i in s {
            let i = i as usize;
            line[i] = b'Q';
            res.push(String::from_utf8(line.clone()).unwrap());
            line[i] = b'.';
        }
        res
    }
    fn check(s: &Vec<i32>, idx: usize) -> bool {
        if idx == 0 {
            return true;
        }
        for i in 0..idx {
            if s[i] == s[idx] {
                return false;
            }
            if (idx - i) as i32 == (s[i] - s[idx]).abs() {
                return false;
            }
        }
        true
    }
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut solv = vec![0; n];
        let mut res = vec![];
        let mut col = 0;
        let mut row = 0;

        loop {
            solv[col] = row;
            if Self::check(&solv, col) {
                if col == n - 1 {
                    res.push(Self::conv(&solv));
                    row += 1;
                } else {
                    col += 1;
                    row = 0;
                }
            } else {
                row += 1;
            }
            while row == n as i32 {
                if col == 0 {
                    return res;
                }
                col -= 1;
                row = solv[col] + 1;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vec_str, vec_vec_str};
    #[test]
    fn solve_n_queens() {
        assert_eq!(
            Solution::solve_n_queens(4),
            vec_vec_str![
                [".Q..", "...Q", "Q...", "..Q."],
                ["..Q.", "Q...", "...Q", ".Q.."]
            ]
        );
        assert_eq!(Solution::solve_n_queens(1), vec_vec_str![["Q"]]);
    }
}
