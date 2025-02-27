// https://leetcode.com/problems/search-a-2d-matrix-ii/
// 240. Search a 2D Matrix II
pub struct Solution;
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let mut col = matrix[0].partition_point(|&x| x <= target);
        if col == 0 {
            return false;
        }
        col -= 1;
        let mut row = 0;
        loop {
            while row < m && matrix[row][col] < target {
                row += 1;
            }
            if row == m {
                return false;
            }
            if matrix[row][col] == target {
                return true;
            }
            while col > 0 && matrix[row][col] > target {
                col -= 1;
            }
            if matrix[row][col] == target {
                return true;
            }
            if matrix[row][col] > target {
                return false;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn search_matrix() {
        assert_eq!(
            Solution::search_matrix(
                vec_vec![
                    [1, 4, 7, 11, 15],
                    [2, 5, 8, 12, 19],
                    [3, 6, 9, 16, 22],
                    [10, 13, 14, 17, 24],
                    [18, 21, 23, 26, 30]
                ],
                5
            ),
            true
        );
        assert_eq!(
            Solution::search_matrix(
                vec![
                    vec![1, 4, 7, 11, 15],
                    vec![2, 5, 8, 12, 19],
                    vec![3, 6, 9, 16, 22],
                    vec![10, 13, 14, 17, 24],
                    vec![18, 21, 23, 26, 30]
                ],
                20
            ),
            false
        );
    }
}
