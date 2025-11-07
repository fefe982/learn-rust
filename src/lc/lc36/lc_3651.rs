// https://leetcode.com/problems/minimum-cost-path-with-teleportations/
// 3651. Minimum Cost Path With Teleportations
pub struct Solution;
impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let ii = grid.len();
        let jj = grid[0].len();
        let mut c = vec![vec![i32::MAX - 10000; jj]; ii];
        c[ii - 1][jj - 1] = 0;
        let &mx = grid.iter().flatten().max().unwrap();
        let mut v_min_cost = vec![i32::MAX; mx as usize + 2];
        let mut c_min_cost = vec![i32::MAX; mx as usize + 2];
        for _ in 0..=k {
            for i in (0..ii).rev() {
                for j in (0..jj).rev() {
                    if j < jj - 1 {
                        c[i][j] = c[i][j].min(c[i][j + 1] + grid[i][j + 1]);
                    }
                    if i < ii - 1 {
                        c[i][j] = c[i][j].min(c[i + 1][j] + grid[i + 1][j]);
                    }
                    c[i][j] = c[i][j].min(v_min_cost[grid[i][j] as usize + 1]);
                    c_min_cost[grid[i][j] as usize + 1] = c_min_cost[grid[i][j] as usize + 1].min(c[i][j]);
                }
            }
            let mut update = false;
            for i in 0..=mx as usize {
                let nv = v_min_cost[i].min(v_min_cost[i + 1]).min(c_min_cost[i + 1]);
                if v_min_cost[i + 1] > nv {
                    update = true;
                    v_min_cost[i + 1] = nv;
                }
            }
            if !update {
                break;
            }
        }
        c[0][0]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_cost() {
        assert_eq!(
            Solution::min_cost(vec_vec![[1, 6, 3, 2, 6, 0], [9, 2, 12, 3, 3, 14]], 5),
            14
        );
        assert_eq!(Solution::min_cost(vec_vec![[1, 3, 3], [2, 5, 4], [4, 3, 5]], 2), 7);
        assert_eq!(Solution::min_cost(vec_vec![[1, 2], [2, 3], [3, 4]], 1), 9);
    }
}
