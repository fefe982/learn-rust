// https://leetcode.com/problems/construct-product-matrix/
// 2906. Construct Product Matrix
pub struct Solution;
impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = vec![vec![0; n]; m];
        let mut prod = 1;
        for i in 0..m {
            for j in 0..n {
                ans[i][j] = prod;
                prod = (prod * (grid[i][j] % 12345)) % 12345;
            }
        }
        prod = 1;
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                ans[i][j] = (ans[i][j] * prod) % 12345;
                prod = (prod * (grid[i][j] % 12345)) % 12345;
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
    fn construct_product_matrix() {
        assert_eq!(
            Solution::construct_product_matrix(vec_vec![[1, 2], [3, 4]]),
            vec_vec![[24, 12], [8, 6]]
        );
        assert_eq!(
            Solution::construct_product_matrix(vec_vec![[12345], [2], [1]]),
            vec_vec![[2], [0], [0]]
        );
    }
}
