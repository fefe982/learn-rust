// https://leetcode.com/problems/cells-with-odd-values-in-a-matrix/
// 1252. Cells with Odd Values in a Matrix
pub struct Solution;
impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut vm = vec![0; m as usize];
        let mut vn = vec![0; n as usize];
        for v in indices {
            vm[v[0] as usize] += 1;
            vn[v[1] as usize] += 1;
        }
        let mut om = 0;
        let mut on = 0;
        for i in 0..m as usize {
            om += vm[i] & 1;
        }
        for i in 0..n as usize {
            on += vn[i] & 1;
        }
        om * (n - on) + on * (m - om)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn odd_cells() {
        assert_eq!(Solution::odd_cells(2, 3, vec_vec![[0, 1], [1, 1]]), 6);
        assert_eq!(Solution::odd_cells(2, 2, vec_vec![[1, 1], [0, 0]]), 0);
    }
}
