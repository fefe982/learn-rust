// https://leetcode.com/problems/island-perimeter/
// 463. Island Perimeter
pub struct Solution;
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    ans += 4;
                    if i > 0 && grid[i - 1][j] == 1 {
                        ans -= 2;
                    }
                    if j > 0 && grid[i][j - 1] == 1 {
                        ans -= 2;
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
    fn test_island_perimeter() {
        assert_eq!(
            Solution::island_perimeter(vec_vec![[0, 1, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [1, 1, 0, 0]]),
            16
        );
        assert_eq!(Solution::island_perimeter(vec_vec![[1]]), 4);
        assert_eq!(Solution::island_perimeter(vec_vec![[1, 0]]), 4);
    }
}
