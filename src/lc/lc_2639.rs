// https://leetcode.com/problems/find-the-width-of-columns-of-a-grid/
// 2639. Find the Width of Columns of a Grid
pub struct Solution;
impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        fn width(i: i32) -> i32 {
            if i == 0 {
                return 1;
            }
            let mut j = i.abs();
            let mut w = if i < 0 { 1 } else { 0 };
            while j > 0 {
                w += 1;
                j /= 10;
            }
            w
        }
        let mut ans = vec![0; grid[0].len()];
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                ans[j] = std::cmp::max(ans[j], width(grid[i][j]));
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
    fn test_find_column_width() {
        assert_eq!(Solution::find_column_width(vec_vec![[1], [22], [333]]), [3]);
        assert_eq!(
            Solution::find_column_width(vec_vec![[-15, 1, 3], [15, 7, 12], [5, 6, -2]]),
            [3, 1, 2]
        );
    }
}
