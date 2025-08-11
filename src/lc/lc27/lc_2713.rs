// https://leetcode.com/problems/maximum-strictly-increasing-cells-in-a-matrix/
// 2713. Maximum Strictly Increasing Cells in a Matrix
pub struct Solution;
impl Solution {
    pub fn max_increasing_cells(mat: Vec<Vec<i32>>) -> i32 {
        let mut map = std::collections::BTreeMap::<i32, Vec<(usize, usize)>>::new();
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                map.entry(mat[i][j]).or_default().push((i, j));
            }
        }
        let mut iv = vec![0; mat.len()];
        let mut jv = vec![0; mat[0].len()];
        for (_, v) in map {
            let nrank = v.iter().map(|&(i, j)| iv[i].max(jv[j]) + 1).collect::<Vec<_>>();
            for (rank, (i, j)) in nrank.into_iter().zip(v) {
                iv[i] = iv[i].max(rank);
                jv[j] = jv[j].max(rank);
            }
        }
        iv.into_iter().max().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_increasing_cells() {
        assert_eq!(Solution::max_increasing_cells(vec_vec![[1, -8], [4, 4], [-3, -9]]), 4);
        assert_eq!(Solution::max_increasing_cells(vec_vec![[3, 1], [3, 4]]), 2);
        assert_eq!(Solution::max_increasing_cells(vec_vec![[1, 1], [1, 1]]), 1);
        assert_eq!(Solution::max_increasing_cells(vec_vec![[3, 1, 6], [-9, 5, 7]]), 4);
    }
}
