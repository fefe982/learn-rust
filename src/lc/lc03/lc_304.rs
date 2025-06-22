// https://leetcode.com/problems/range-sum-query-2d-immutable/
// 304. Range Sum Query 2D - Immutable
pub struct NumMatrix {
    matrix: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut v = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                v[i + 1][j + 1] = v[i + 1][j] + v[i][j + 1] - v[i][j] + matrix[i][j];
            }
        }
        Self { matrix: v }
    }

    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.matrix[row2 as usize + 1][col2 as usize + 1]
            - self.matrix[row2 as usize + 1][col1 as usize]
            - self.matrix[row1 as usize][col2 as usize + 1]
            + self.matrix[row1 as usize][col1 as usize]
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_it_works() {
        let obj = NumMatrix::new(vec_vec![
            [3, 0, 1, 4, 2],
            [5, 6, 3, 2, 1],
            [1, 2, 0, 1, 5],
            [4, 1, 0, 1, 7],
            [1, 0, 3, 0, 5]
        ]);
        assert_eq!(obj.sum_region(2, 1, 4, 3), 8);
        assert_eq!(obj.sum_region(1, 1, 2, 2), 11);
        assert_eq!(obj.sum_region(1, 2, 2, 4), 12);
    }
}
