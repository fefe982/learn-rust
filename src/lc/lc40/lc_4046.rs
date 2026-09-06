// https://leetcode.com/problems/minimum-cost-path-with-at-most-k-turns/
// 4046. Minimum Cost Path With at Most K Turns
pub struct Solution;
impl Solution {
    fn walk(
        grid: &Vec<Vec<i32>>,
        k_left: usize,
        x: usize,
        y: usize,
        dir: usize,
        cache: &mut Vec<Vec<Vec<Vec<i32>>>>,
    ) -> i32 {
        if cache[k_left][x][y][dir] != -1 {
            return cache[k_left][x][y][dir];
        }
        if x == grid.len() - 1 && y == grid[0].len() - 1 {
            return grid[x][y];
        }
        let mut min = i32::MAX;
        for (id, &(dx, dy)) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter().enumerate() {
            if k_left == 0 && id != dir {
                continue;
            }
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if nx >= grid.len() || ny >= grid[0].len() {
                continue;
            }
            let nstep = if id == dir { 0 } else { 1 };
            let n = Self::walk(grid, k_left - nstep, nx, ny, id, cache);
            min = min.min(n);
        }
        if min != i32::MAX {
            min += grid[x][y];
        }
        cache[k_left][x][y][dir] = min;
        min
    }
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let nx = grid.len();
        let ny = grid[0].len();
        let k = k as usize;
        let mut cache = vec![vec![vec![vec![-1; 4]; ny]; nx]; k + 1];
        let d1 = Self::walk(&grid, k, 0, 0, 0, &mut cache);
        let d2 = Self::walk(&grid, k, 0, 0, 1, &mut cache);
        let d = d1.min(d2);
        if d == i32::MAX {
            -1
        } else {
            d
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_cost() {
        assert_eq!(Solution::min_cost(vec_vec![[2, 7, 3], [1, 4, 5]], 1), 12);
        assert_eq!(Solution::min_cost(vec_vec![[4, 1, 9], [3, 2, 5], [4, 8, 6]], 2), 20);
        assert_eq!(Solution::min_cost(vec_vec![[1, 9], [3, 4]], 0), -1);
    }
}
