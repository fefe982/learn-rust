// https://leetcode.cn/problems/eight-queens-lcci/
// 面试题 08.12. Eight Queens LCCI
pub struct Solution;
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        super::super::lc00::lc_51::Solution::solve_n_queens(n)
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
            vec_vec_str![[".Q..", "...Q", "Q...", "..Q."], ["..Q.", "Q...", "...Q", ".Q.."]]
        );
        assert_eq!(Solution::solve_n_queens(1), vec_vec_str![["Q"]]);
    }
}
