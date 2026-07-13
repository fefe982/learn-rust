// https://leetcode.com/problems/sum-of-matrix-after-queries/
// 2718. Sum of Matrix After Queries
pub struct Solution;
impl Solution {
    pub fn matrix_sum_queries(n: i32, queries: Vec<Vec<i32>>) -> i64 {
        let mut fc = vec![true; n as usize];
        let mut nc = n as i64;
        let mut fr = vec![true; n as usize];
        let mut nr = n as i64;
        let mut ans = 0;
        for q in queries.iter().rev() {
            if q[0] == 0 {
                let ir = q[1] as usize;
                if fr[ir] {
                    ans += q[2] as i64 * nr;
                    fr[ir] = false;
                    nc -= 1;
                }
            } else {
                let ic = q[1] as usize;
                if fc[ic] {
                    ans += q[2] as i64 * nc as i64;
                    fc[ic] = false;
                    nr -= 1;
                }
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
    fn matrix_sum_queries() {
        assert_eq!(
            Solution::matrix_sum_queries(3, vec_vec![[0, 0, 1], [1, 2, 2], [0, 2, 3], [1, 0, 4]]),
            23
        );
        assert_eq!(
            Solution::matrix_sum_queries(3, vec_vec![[0, 0, 4], [0, 1, 2], [1, 0, 1], [0, 2, 3], [1, 2, 1]]),
            17
        );
    }
}
