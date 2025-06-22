// https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/
// 363. Max Sum of Rectangle No Larger Than K
pub struct Solution;
impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut max = i32::MIN;
        for l in 0..matrix.len() {
            let mut t = vec![0; matrix[l].len()];
            for r in l..matrix.len() {
                let mut sum = std::collections::BTreeSet::new();
                sum.insert(0);
                let mut rsum = 0;
                for i in 0..matrix[0].len() {
                    t[i] += matrix[r][i];
                    rsum += t[i];
                    if let Some(&lsum) = sum.range((rsum - k)..).next() {
                        max = max.max(rsum - lsum);
                    }
                    sum.insert(rsum);
                }
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_sum_submatrix() {
        assert_eq!(
            Solution::max_sum_submatrix(vec_vec![[1, 0, 1], [0, -2, 3]], 2),
            2
        );
        assert_eq!(Solution::max_sum_submatrix(vec_vec![[2, 2, -1]], 3), 3);
    }
}
