// https://leetcode.com/problems/increment-submatrices-by-one/
// 2536. Increment Submatrices by One
pub struct Solution;
impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut mat = vec![vec![0; n]; n];
        for q in queries {
            let (r1, c1, r2, c2) = (q[0] as usize, q[1] as usize, q[2] as usize, q[3] as usize);
            mat[r1][c1] += 1;
            if r2 + 1 < n {
                mat[r2 + 1][c1] -= 1;
            }
            if c2 + 1 < n {
                mat[r1][c2 + 1] -= 1;
            }
            if r2 + 1 < n && c2 + 1 < n {
                mat[r2 + 1][c2 + 1] += 1;
            }
        }
        for j in 1..n {
            mat[0][j] += mat[0][j - 1];
        }
        for i in 1..n {
            mat[i][0] += mat[i - 1][0];
            for j in 1..n {
                mat[i][j] += mat[i - 1][j] + mat[i][j - 1] - mat[i - 1][j - 1];
            }
        }
        mat
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn range_add_queries() {
        assert_eq!(
            Solution::range_add_queries(3, vec_vec![[1, 1, 2, 2], [0, 0, 1, 1]]),
            vec_vec![[1, 1, 0], [1, 2, 1], [0, 1, 1]]
        );
        assert_eq!(
            Solution::range_add_queries(2, vec_vec![[0, 0, 1, 1]]),
            vec_vec![[1, 1], [1, 1]]
        );
    }
}
