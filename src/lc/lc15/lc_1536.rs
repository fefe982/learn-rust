// https://leetcode.com/problems/minimum-swaps-to-arrange-a-binary-grid/
// 1536. Minimum Swaps to Arrange a Binary Grid
pub struct Solution;
impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let mut trail = grid
            .into_iter()
            .map(|row| row.into_iter().rev().take_while(|&x| x == 0).count())
            .collect::<Vec<_>>();
        let n = trail.len();
        let mut cnt = 0;
        'i: for i in 0..n {
            for j in 0..trail.len() {
                if trail[j] >= n - i - 1 {
                    cnt = cnt + j as i32;
                    trail.remove(j);
                    continue 'i;
                }
            }
            return -1;
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_swaps() {
        assert_eq!(Solution::min_swaps(vec_vec![[0, 0, 1], [1, 1, 0], [1, 0, 0]]), 3);
        assert_eq!(
            Solution::min_swaps(vec_vec![[0, 1, 1, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 1, 1, 0]]),
            -1
        );
        assert_eq!(Solution::min_swaps(vec_vec![[1, 0, 0], [1, 1, 0], [1, 1, 1]]), 0);
    }
}
