// https://leetcode.com/problems/grid-game/
// 2017. Grid Game
pub struct Solution;
impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let mut sum_up = grid[0].iter().map(|x| *x as i64).sum::<i64>();
        let mut sum_down = 0;
        let mut min = i64::MAX;
        for i in 0..grid[0].len() {
            sum_up -= grid[0][i] as i64;
            min = min.min(sum_up.max(sum_down));
            sum_down += grid[1][i] as i64;
        }
        min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn grid_game() {
        assert_eq!(Solution::grid_game(vec_vec![[2, 5, 4], [1, 5, 1]]), 4);
        assert_eq!(Solution::grid_game(vec_vec![[3, 3, 1], [8, 5, 2]]), 4);
        assert_eq!(Solution::grid_game(vec_vec![[1, 3, 1, 15], [1, 3, 3, 1]]), 7);
    }
}
