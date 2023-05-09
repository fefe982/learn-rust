// https://leetcode.com/problems/spiral-matrix/
// 54. Spiral Matrix
pub struct Solution;
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut top = 0;
        let mut bottom = matrix.len();
        let mut left = 0;
        let mut right = matrix[0].len();
        let mut res = Vec::with_capacity(matrix.len() * matrix[0].len());
        loop {
            for i in left..right {
                res.push(matrix[top][i]);
            }
            top += 1;
            if bottom <= top {
                break;
            }
            for i in top..bottom {
                res.push(matrix[i][right - 1]);
            }
            right -= 1;
            if right <= left {
                break;
            }
            for i in (left..right).rev() {
                res.push(matrix[bottom - 1][i]);
            }
            bottom -= 1;
            if bottom <= top {
                break;
            }
            for i in (top..bottom).rev() {
                res.push(matrix[i][left]);
            }
            left += 1;
            if right <= left {
                break;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn spiral_order() {
        assert_eq!(
            Solution::spiral_order(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(
            Solution::spiral_order(vec_vec![[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }
}
