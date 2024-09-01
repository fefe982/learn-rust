// https://leetcode.com/problems/convert-1d-array-into-2d-array/
// 2022. Convert 1D Array Into 2D Array
pub struct Solution;
impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let m = m as usize;
        let n = n as usize;
        if m * n != original.len() {
            return vec![];
        }
        let mut ret = vec![vec![0; n as usize]; m as usize];
        for i in 0..m {
            for j in 0..n {
                ret[i][j] = original[i * n + j];
            }
        }
        ret
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn construct2_d_array() {
        assert_eq!(
            Solution::construct2_d_array(vec![1, 2, 3, 4], 2, 2),
            vec_vec![[1, 2], [3, 4]]
        );
        assert_eq!(Solution::construct2_d_array(vec![1, 2, 3], 1, 3), vec_vec![[1, 2, 3]]);
        assert_eq!(Solution::construct2_d_array(vec![1, 2], 1, 1), Vec::<Vec<i32>>::new());
    }
}
