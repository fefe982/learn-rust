// https://leetcode.com/problems/right-triangles/
// 3128. Right Triangles
pub struct Solution;
impl Solution {
    pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
        let ni = grid.len();
        let nj = grid[0].len();
        let mut cnti = vec![0; ni];
        let mut cntj = vec![0; nj];
        for i in 0..ni {
            for j in 0..nj {
                if grid[i][j] == 1 {
                    cnti[i] += 1;
                    cntj[j] += 1;
                }
            }
        }
        let mut ans = 0;
        for i in 0..ni {
            for j in 0..nj {
                if grid[i][j] == 1 {
                    ans += (cnti[i] - 1) as i64 * (cntj[j] - 1) as i64
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
    fn test_number_of_right_triangles() {
        assert_eq!(
            Solution::number_of_right_triangles(vec_vec![[0, 1, 0], [0, 1, 1], [0, 1, 0]]),
            2
        );
        assert_eq!(
            Solution::number_of_right_triangles(vec_vec![[1, 0, 0, 0], [0, 1, 0, 1], [1, 0, 0, 0]]),
            0
        );
        assert_eq!(
            Solution::number_of_right_triangles(vec_vec![[1, 0, 1], [1, 0, 0], [1, 0, 0]]),
            2
        );
    }
}
