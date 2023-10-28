// https://leetcode.com/problems/search-a-2d-matrix/
// 74. Search a 2D Matrix
pub struct Solution;
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let pos = matrix.partition_point(|v| v[0] <= target);
        if pos == 0 {
            return false;
        }
        if let Ok(_) = matrix[pos - 1].binary_search(&target) {
            true
        } else {
            false
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_window() {
        assert_eq!(
            Solution::search_matrix(
                vec_vec![[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]],
                3
            ),
            true
        );
        assert_eq!(
            Solution::search_matrix(
                vec_vec![[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]],
                13
            ),
            false
        );
    }
}
