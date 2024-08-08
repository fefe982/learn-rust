// https://leetcode.com/problems/spiral-matrix-iii/
// 885. Spiral Matrix III
pub struct Solution;
impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![r_start, c_start]];
        let dir = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let inc = [1, 0, 1, 0];
        let mut l = 1;
        let mut cdir = 0;
        let mut r = r_start;
        let mut c = c_start;
        while ans.len() < (rows * cols) as usize {
            let (dr, dc) = dir[cdir];
            for _ in 0..l {
                r += dr;
                c += dc;
                if r >= 0 && r < rows && c >= 0 && c < cols {
                    ans.push(vec![r, c]);
                }
            }
            cdir = (cdir + 1) % 4;
            l += inc[cdir];
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_spiral_matrix_iii() {
        assert_eq!(
            Solution::spiral_matrix_iii(1, 4, 0, 0),
            vec_vec![[0, 0], [0, 1], [0, 2], [0, 3]]
        )
    }
}
