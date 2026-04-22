// https://leetcode.com/problems/rotating-the-box/
// 1861. Rotating the Box
pub struct Solution;
impl Solution {
    pub fn rotate_the_box(box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut ans = vec![vec!['.'; box_grid.len()]; box_grid[0].len()];
        let l = box_grid.len();
        for i in 0..box_grid.len() {
            let mut nx = box_grid[i].len();
            for j in (0..box_grid[i].len()).rev() {
                if box_grid[i][j] == '*' {
                    nx = j;
                    ans[j][l - i - 1] = '*';
                } else if box_grid[i][j] == '#' {
                    nx -= 1;
                    ans[nx][l - i - 1] = '#';
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
    fn test_rotate_the_box() {
        assert_eq!(
            Solution::rotate_the_box(vec_vec_chr![["#", ".", "#"]]),
            vec_vec_chr![["."], ["#"], ["#"]]
        );
        assert_eq!(
            Solution::rotate_the_box(vec_vec_chr![["#", ".", "*", "."], ["#", "#", "*", "."]]),
            vec_vec_chr![["#", "."], ["#", "#"], ["*", "*"], [".", "."]]
        );
        assert_eq!(
            Solution::rotate_the_box(vec_vec_chr![
                ["#", "#", "*", ".", "*", "."],
                ["#", "#", "#", "*", ".", "."],
                ["#", "#", "#", ".", "#", "."]
            ]),
            vec_vec_chr![
                [".", "#", "#"],
                [".", "#", "#"],
                ["#", "#", "*"],
                ["#", "*", "."],
                ["#", ".", "*"],
                ["#", ".", "."]
            ]
        );
    }
}
