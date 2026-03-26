// https://leetcode.com/problems/matrix-similarity-after-cyclic-shifts/
// 2946. Matrix Similarity After Cyclic Shifts
pub struct Solution;
impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let n = mat[0].len();
        let shift = (k as usize) % n;

        for (i, row) in mat.iter().enumerate() {
            for j in 0..n {
                let shifted_idx = if i % 2 == 0 {
                    (j + shift) % n
                } else {
                    (j + n - shift) % n
                };

                if row[j] != row[shifted_idx] {
                    return false;
                }
            }
        }

        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn are_similar() {
        assert_eq!(
            Solution::are_similar(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]], 4),
            false
        );
        assert_eq!(
            Solution::are_similar(vec_vec![[1, 2, 1, 2], [5, 5, 5, 5], [6, 3, 6, 3]], 2),
            true
        );
        assert_eq!(Solution::are_similar(vec_vec![[2, 2], [2, 2]], 3), true);
    }
}
