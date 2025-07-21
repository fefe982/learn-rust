// https://leetcode.com/problems/rank-transform-of-a-matrix/
// 1632. Rank Transform of a Matrix
pub struct Solution;
impl Solution {
    fn get_p(p: &mut Vec<usize>, x: usize) -> usize {
        let mut px = x;
        while px != p[px] {
            px = p[px];
        }
        p[x] = px;
        px
    }
    fn union(p: &mut Vec<usize>, x: usize, y: usize) {
        let px = Self::get_p(p, x);
        let py = Self::get_p(p, y);
        p[px] = py;
    }
    pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut vals = std::collections::BTreeMap::<i32, Vec<(usize, usize)>>::new();
        for i in 0..m {
            for j in 0..n {
                let v = matrix[i][j];
                vals.entry(v).or_default().push((i, j));
            }
        }
        let mut rank = vec![0; m + n];
        let mut ans = vec![vec![0; n]; m];
        for (_, pts) in vals {
            let mut p = (0..(m + n)).collect::<Vec<_>>();
            for &(x, y) in &pts {
                Self::union(&mut p, x, y + m);
            }
            let mut g = std::collections::HashMap::<usize, i32>::new();
            for &(x, y) in &pts {
                let px = Self::get_p(&mut p, x);
                let r = g.entry(px).or_default();
                *r = (*r).max(rank[x]).max(rank[y + m]);
            }
            for &(x, y) in &pts {
                let px = Self::get_p(&mut p, x);
                rank[x] = g[&px] + 1;
                rank[y + m] = g[&px] + 1;
                ans[x][y] = rank[x];
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
    fn test_matix_rank_transform() {
        assert_eq!(
            Solution::matrix_rank_transform(vec_vec![[1, 2], [3, 4]]),
            vec_vec![[1, 2], [2, 3]]
        );
        assert_eq!(
            Solution::matrix_rank_transform(vec_vec![[7, 7], [7, 7]]),
            vec_vec![[1, 1], [1, 1]]
        );
        assert_eq!(
            Solution::matrix_rank_transform(vec_vec![[20, -21, 14], [-19, 4, 19], [22, -47, 24], [-19, 4, 19]]),
            vec_vec![[4, 2, 3], [1, 3, 4], [5, 1, 6], [1, 3, 4]]
        );
    }
}
