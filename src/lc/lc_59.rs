// https://leetcode.com/problems/spiral-matrix-ii/
// 59. Spiral Matrix II
pub struct Solution;
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut res = vec![vec![0; n]; n];
        let mut m = 1;
        let mut l = 0;
        let mut r = n;
        let mut t = 0;
        let mut b = n;
        loop {
            for i in l..r {
                res[t][i] = m;
                m += 1;
            }
            t += 1;
            if t >= b {
                break;
            }
            for i in t..b {
                res[i][r - 1] = m;
                m += 1;
            }
            r -= 1;
            if l >= r {
                break;
            }
            for i in (l..r).rev() {
                res[b - 1][i] = m;
                m += 1;
            }
            b -= 1;
            if t >= b {
                break;
            }
            for i in (t..b).rev() {
                res[i][l] = m;
                m += 1;
            }
            l += 1;
            if l >= r {
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
    fn generate_matrix() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec_vec![[1, 2, 3], [8, 9, 4], [7, 6, 5]]
        );
        assert_eq!(Solution::generate_matrix(1), vec_vec![[1]]);
    }
}
