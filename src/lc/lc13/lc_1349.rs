// https://leetcode.com/problems/maximum-students-taking-exam/
// 1349. Maximum Students Taking Exam
pub struct Solution;
impl Solution {
    fn max_student_helper(
        state: u64,
        nrow: usize,
        ncol: usize,
        cache: &mut std::collections::HashMap<u64, i32>,
    ) -> i32 {
        if let Some(&v) = cache.get(&state) {
            return v;
        }
        let pos_s = state.trailing_zeros();
        let seat = 1 << pos_s;
        let mut m = Self::max_student_helper(state & !seat, nrow, ncol, cache);
        let r = pos_s as usize / ncol;
        let row = ((1 << ncol) - 1) << (r * ncol);
        let lr = ((seat << 1) | (seat >> 1)) & row;
        let nlr = if r + 1 < nrow { lr << ncol } else { 0 };
        m = m.max(Self::max_student_helper(state & !lr & !seat & !nlr, nrow, ncol, cache) + 1);
        cache.insert(state, m);
        m
    }
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        let nrow = seats.len();
        let ncol = seats[0].len();
        let mut state = 0u64;
        for i in 0..nrow {
            for j in 0..ncol {
                if seats[i][j] == '.' {
                    state |= 1 << (i * ncol + j);
                }
            }
        }
        let mut cache = std::collections::HashMap::new();
        cache.insert(0, 0);
        Solution::max_student_helper(state, nrow, ncol, &mut cache)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_student() {
        assert_eq!(
            Solution::max_students(vec_vec_chr![
                ["#", ".", "#", "#", ".", "#"],
                [".", "#", "#", "#", "#", "."],
                ["#", ".", "#", "#", ".", "#"]
            ]),
            4
        );
        assert_eq!(
            Solution::max_students(vec_vec_chr![[".", "#"], ["#", "#"], ["#", "."], ["#", "#"], [".", "#"]]),
            3
        );
        assert_eq!(
            Solution::max_students(vec_vec_chr![
                ["#", ".", ".", ".", "#"],
                [".", "#", ".", "#", "."],
                [".", ".", "#", ".", "."],
                [".", "#", ".", "#", "."],
                ["#", ".", ".", ".", "#"]
            ]),
            10
        );
    }
}
