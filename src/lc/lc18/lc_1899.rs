// https://leetcode.com/problems/merge-triplets-to-form-target-triplet/
// 1899. Merge Triplets to Form Target Triplet
pub struct Solution;
impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let mut found = [false; 3];
        for triplet in triplets {
            if triplet[0] > target[0] || triplet[1] > target[1] || triplet[2] > target[2] {
                continue;
            }
            for i in 0..3 {
                if triplet[i] == target[i] {
                    found[i] = true;
                }
            }
        }
        found.into_iter().all(|x| x)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_merge_triplets() {
        assert_eq!(
            Solution::merge_triplets(vec_vec![[2, 5, 3], [1, 8, 4], [1, 7, 5]], vec![2, 7, 5]),
            true
        );
        assert_eq!(
            Solution::merge_triplets(vec_vec![[3, 4, 5], [4, 5, 6]], vec![3, 2, 5]),
            false
        );
        assert_eq!(
            Solution::merge_triplets(vec_vec![[2, 5, 3], [2, 3, 4], [1, 2, 5], [5, 2, 3]], vec![5, 5, 5]),
            true
        );
    }
}
