// https://leetcode.com/problems/count-unguarded-cells-in-the-grid/
// 2257. Count Unguarded Cells in the Grid
pub struct Solution;
impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut grid = vec![vec![0; n]; m];
        for g in &guards {
            grid[g[0] as usize][g[1] as usize] = 2;
        }
        for w in walls {
            grid[w[0] as usize][w[1] as usize] = 2;
        }
        for g in guards {
            let (x, y) = (g[0] as usize, g[1] as usize);
            for i in x + 1..m {
                if grid[i][y] == 2 {
                    break;
                }
                grid[i][y] = 1;
            }
            for i in (0..x).rev() {
                if grid[i][y] == 2 {
                    break;
                }
                grid[i][y] = 1;
            }
            for i in y + 1..n {
                if grid[x][i] == 2 {
                    break;
                }
                grid[x][i] = 1;
            }
            for i in (0..y).rev() {
                if grid[x][i] == 2 {
                    break;
                }
                grid[x][i] = 1;
            }
        }
        grid.into_iter().flat_map(|r| r.into_iter()).filter(|&x| x == 0).count() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_unguarded() {
        assert_eq!(
            Solution::count_unguarded(4, 6, vec_vec![[0, 0], [1, 1], [2, 3]], vec_vec![[0, 1], [2, 2], [1, 4]]),
            7
        );
        assert_eq!(
            Solution::count_unguarded(3, 3, vec_vec![[1, 1]], vec_vec![[0, 1], [1, 0], [2, 1], [1, 2]]),
            4
        );
    }
}
