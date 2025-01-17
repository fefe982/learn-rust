// https://leetcode.com/problems/permutations-ii/
// 47. Permutations II
pub struct Solution;
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        super::lc_46::Solution::permute(nums)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn my_pow() {
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 2]),
            vec_vec![[1, 1, 2], [1, 2, 1], [2, 1, 1]]
        );
        assert_eq!(
            Solution::permute_unique(vec![1, 2, 3]),
            vec_vec![[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]
        );
        assert_eq!(Solution::permute_unique(vec![0, 1]), vec_vec![[0, 1], [1, 0]]);
        assert_eq!(Solution::permute_unique(vec![1]), vec_vec![[1]]);
    }
}
