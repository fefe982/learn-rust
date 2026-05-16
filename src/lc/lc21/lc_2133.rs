// https://leetcode.com/problems/check-if-every-row-and-column-contains-all-numbers/
// 2133. Check if Every Row and Column Contains All Numbers
pub struct Solution;
impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();
        for i in 0..n {
            let mut row = vec![false; n];
            let mut col = vec![false; n];
            for j in 0..n {
                if row[(matrix[i][j] - 1) as usize] {
                    return false;
                }
                row[(matrix[i][j] - 1) as usize] = true;
                if col[(matrix[j][i] - 1) as usize] {
                    return false;
                }
                col[(matrix[j][i] - 1) as usize] = true;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_check_valid() {
        assert_eq!(Solution::check_valid(vec_vec![[1, 2, 3], [2, 3, 1], [3, 1, 2]]), true);
        assert_eq!(Solution::check_valid(vec_vec![[1, 1, 1], [1, 2, 3], [1, 2, 3]]), false);
    }
}
