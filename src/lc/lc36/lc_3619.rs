// https://leetcode.com/problems/count-islands-with-total-value-divisible-by-k/
// 3619. Count Islands With Total Value Divisible by K
pub struct Solution;
impl Solution {
    pub fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut ans = 0;
        let mut grid = grid;
        let k = k as i64;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != 0 {
                    let mut sum = grid[i][j] as i64;
                    grid[i][j] = 0;
                    let mut stack = vec![(i, j)];
                    while let Some((x, y)) = stack.pop() {
                        for (xx, yy) in [(x + 1, y), (x.wrapping_sub(1), y), (x, y + 1), (x, y.wrapping_sub(1))] {
                            if xx < grid.len() && yy < grid[0].len() && grid[xx][yy] != 0 {
                                sum += grid[xx][yy] as i64;
                                grid[xx][yy] = 0;
                                stack.push((xx, yy));
                            }
                        }
                    }
                    if sum % k == 0 {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_islands() {
        assert_eq!(
            Solution::count_islands(
                vec_vec![
                    [0, 2, 1, 0, 0],
                    [0, 5, 0, 0, 5],
                    [0, 0, 1, 0, 0],
                    [0, 1, 4, 7, 0],
                    [0, 2, 0, 0, 8]
                ],
                5
            ),
            2
        );
        assert_eq!(
            Solution::count_islands(vec_vec![[3, 0, 3, 0], [0, 3, 0, 3], [3, 0, 3, 0]], 3),
            6
        );
    }
}
