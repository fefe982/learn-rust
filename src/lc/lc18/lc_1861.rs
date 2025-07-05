// https://leetcode.com/problems/rotating-the-box/
// 1861. Rotating the Box
pub struct Solution;
impl Solution {
    pub fn rotate_the_box(bx: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut ans = vec![vec!['.'; bx.len()]; bx[0].len()];
        let l = bx.len();
        for i in 0..bx.len() {
            let mut nx = bx[i].len();
            for j in (0..bx[i].len()).rev() {
                if bx[i][j] == '*' {
                    nx = j;
                    ans[j][l - i - 1] = '*';
                } else if bx[i][j] == '#' {
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
