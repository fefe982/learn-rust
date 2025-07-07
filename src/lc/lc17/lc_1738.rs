// https://leetcode.com/problems/find-kth-largest-xor-coordinate-value/description
// 1738. Find Kth Largest XOR Coordinate Value
pub struct Solution;
impl Solution {
    pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut h = std::collections::BinaryHeap::<std::cmp::Reverse<i32>>::new();
        let mut matrix = matrix;
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if i > 0 {
                    matrix[i][j] ^= matrix[i - 1][j];
                }
                if j > 0 {
                    matrix[i][j] ^= matrix[i][j - 1];
                }
                if i > 0 && j > 0 {
                    matrix[i][j] ^= matrix[i - 1][j - 1];
                }
                h.push(std::cmp::Reverse(matrix[i][j]));
                if h.len() > k as usize {
                    h.pop();
                }
            }
        }
        h.peek().unwrap().0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_kth_largest_value() {
        assert_eq!(Solution::kth_largest_value(vec_vec![[5, 2], [1, 6]], 1), 7);
        assert_eq!(Solution::kth_largest_value(vec_vec![[5, 2], [1, 6]], 2), 5);
        assert_eq!(Solution::kth_largest_value(vec_vec![[5, 2], [1, 6]], 3), 4);
    }
}
