// https://leetcode.com/problems/snakes-and-ladders/
// 909. Snakes and Ladders
pub struct Solution;
impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len() * board.len();
        let mut v = Vec::with_capacity(n + 1);
        v.push(-1);
        for (i, row) in board.into_iter().rev().enumerate() {
            if i & 1 == 0 {
                v.extend(row.into_iter());
            } else {
                v.extend(row.into_iter().rev());
            }
        }
        let mut visited = vec![false; n + 1];
        let mut q = std::collections::VecDeque::new();
        q.push_back((0, 1));
        visited[1] = true;
        while let Some((step, pos)) = q.pop_front() {
            for i in 1..=6 {
                let mut next_pos = pos + i;
                if v[next_pos] != -1 {
                    next_pos = v[next_pos] as usize
                }
                if next_pos == n {
                    return step + 1;
                }
                if !visited[next_pos] {
                    visited[next_pos] = true;
                    q.push_back((step + 1, next_pos));
                }
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
    fn snakes_and_ladders() {
        assert_eq!(
            Solution::snakes_and_ladders(vec_vec![[-1, -1, -1], [-1, 9, 8], [-1, 8, 9]]),
            1
        );
        assert_eq!(
            Solution::snakes_and_ladders(vec_vec![
                [-1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, -1],
                [-1, 35, -1, -1, 13, -1],
                [-1, -1, -1, -1, -1, -1],
                [-1, 15, -1, -1, -1, -1]
            ]),
            4
        );
        assert_eq!(Solution::snakes_and_ladders(vec_vec![[-1, -1], [-1, 3]]), 1);
    }
}
