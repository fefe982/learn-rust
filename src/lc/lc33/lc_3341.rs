// https://leetcode.com/problems/find-minimum-time-to-reach-last-room-i/
// 3341. Find Minimum Time to Reach Last Room
pub struct Solution;
impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let n = move_time.len();
        let m = move_time[0].len();
        let mut time = vec![vec![i32::MAX; m]; n];
        time[0][0] = 0;
        let mut q = std::collections::BinaryHeap::new();
        q.push(std::cmp::Reverse((0, 0, 0)));
        while let Some(std::cmp::Reverse((t, i, j))) = q.pop() {
            if i == n - 1 && j == m - 1 {
                return t;
            }
            if t > time[i][j] {
                continue;
            }
            time[i][j] = 0;
            for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let ni = (i as i32 + di) as usize;
                let nj = (j as i32 + dj) as usize;
                if ni < n && nj < m {
                    let nt = t.max(move_time[ni][nj]) + 1;
                    if nt < time[ni][nj] {
                        time[ni][nj] = nt;
                        q.push(std::cmp::Reverse((nt, ni, nj)));
                    }
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
    fn min_time_to_reach() {
        assert_eq!(Solution::min_time_to_reach(vec_vec![[0, 4], [4, 4]]), 6);
        assert_eq!(Solution::min_time_to_reach(vec_vec![[0, 0, 0], [0, 0, 0]]), 3);
        assert_eq!(Solution::min_time_to_reach(vec_vec![[0, 1], [1, 2]]), 3);
    }
}
