// https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/
// 1351. Count Negative Numbers in a Sorted Matrix
pub struct Solution;
impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();
        let mut i = grid.len() - 1;
        let mut j = 0;
        let mut cnt = 0;
        loop {
            while j < n && grid[i][j] >= 0 {
                j += 1;
            }
            if j >= n {
                return cnt;
            }
            cnt += (n - j) as i32;
            if i == 0 {
                return cnt;
            }
            i -= 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_negatives() {
        assert_eq!(
            Solution::count_negatives(vec_vec![
                [4, 3, 2, -1],
                [3, 2, 1, -1],
                [1, 1, -1, -2],
                [-1, -1, -2, -3]
            ]),
            8
        );
        assert_eq!(Solution::count_negatives(vec_vec![[3, 2], [1, 0]]), 0);
    }
}
