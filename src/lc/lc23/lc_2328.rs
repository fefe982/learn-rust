// https://leetcode.com/problems/number-of-increasing-paths-in-a-grid/
// 2328. Number of Increasing Paths in a Grid
pub struct Solution;
const MOD: i64 = 1000000007;
#[derive(Copy, Clone)]
struct IMod {
    i: i64,
}
impl IMod {
    fn new(i: i64) -> Self {
        Self { i: i as i64 }
    }
    fn val(self) -> i32 {
        self.i as i32
    }
}
impl std::ops::AddAssign<IMod> for IMod {
    fn add_assign(&mut self, rhs: IMod) {
        return self.i = (self.i + rhs.i) % MOD;
    }
}
impl Solution {
    fn count(grid: &Vec<Vec<i32>>, x: usize, y: usize, cnt: &mut Vec<Vec<IMod>>) -> IMod {
        if cnt[x][y].val() > 0 {
            return cnt[x][y];
        }
        let mut t = IMod::new(1);
        if x > 0 && grid[x - 1][y] > grid[x][y] {
            t += Self::count(grid, x - 1, y, cnt);
        }
        if x + 1 < grid.len() && grid[x + 1][y] > grid[x][y] {
            t += Self::count(grid, x + 1, y, cnt);
        }
        if y > 0 && grid[x][y - 1] > grid[x][y] {
            t += Self::count(grid, x, y - 1, cnt);
        }
        if y + 1 < grid[0].len() && grid[x][y + 1] > grid[x][y] {
            t += Self::count(grid, x, y + 1, cnt);
        }
        cnt[x][y] = t;
        t
    }
    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        let mut cnt = vec![vec![IMod::new(0); grid[0].len()]; grid.len()];
        let mut total = IMod::new(0);
        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                total += Self::count(&grid, x, y, &mut cnt);
            }
        }
        total.val()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_paths() {
        assert_eq!(Solution::count_paths(vec_vec![[1, 1], [3, 4]]), 8);
        assert_eq!(Solution::count_paths(vec_vec![[1], [2]]), 3);
    }
}
