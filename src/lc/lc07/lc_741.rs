// https://leetcode.com/problems/cherry-pickup/
// 741. Cherry Pickup
use std::cmp;
pub struct Solution;
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut cache = vec![vec![vec![0; m]; n]; m];
        for x1 in 0..m {
            for y1 in 0..n {
                for x2 in 0..m {
                    if x2 > x1 + y1 {
                        break;
                    }
                    if x2 + n - 1 < x1 + y1 {
                        continue;
                    }
                    let y2 = x1 + y1 - x2;
                    if grid[x1][y1] == -1 || grid[x2][y2] == -1 {
                        cache[x1][y1][x2] = -1;
                        continue;
                    }
                    let mut last = 0;
                    if x1 > 0 || y1 > 0 {
                        let mut ll = -1;
                        let mut lu = -1;
                        let mut ul = -1;
                        let mut uu = -1;
                        if x1 > 0 && x2 > 0 {
                            ll = cache[x1 - 1][y1][x2 - 1];
                        }
                        if x1 > 0 && y2 > 0 {
                            lu = cache[x1 - 1][y1][x2];
                        }
                        if y1 > 0 && x2 > 0 {
                            ul = cache[x1][y1 - 1][x2 - 1];
                        }
                        if y1 > 0 && y2 > 0 {
                            uu = cache[x1][y1 - 1][x2];
                        }
                        last = cmp::max(cmp::max(ll, lu), cmp::max(ul, uu));
                    }
                    if last < 0 {
                        cache[x1][y1][x2] = -1
                    } else {
                        if x1 == x2 {
                            cache[x1][y1][x1] = last + grid[x1][y1];
                        } else {
                            cache[x1][y1][x2] = last + grid[x1][y1] + grid[x2][y2];
                        }
                    }
                }
            }
        }
        cmp::max(cache[m - 1][n - 1][m - 1], 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cherry_pickup() {
        assert_eq!(
            Solution::cherry_pickup(vec![vec![0, 1, -1], vec![1, 0, -1], vec![1; 3]]),
            5
        );
        assert_eq!(
            Solution::cherry_pickup(vec![vec![1, 1, -1], vec![1, -1, 1], vec![-1, 1, 1]]),
            0
        );
    }
}
