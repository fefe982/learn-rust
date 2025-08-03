// https://leetcode.com/problems/delete-columns-to-make-sorted-ii/
// 955. Delete Columns to Make Sorted II
pub struct Solution;
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut cnt = 0;
        let mut idx = vec![0; strs.len()];
        let n = strs.len();
        let mut sorted = 0;
        for i in 0..strs[0].len() {
            let mut throw = false;
            for j in 1..strs.len() {
                if idx[j] == 0 && strs[j].as_bytes()[i] < strs[j - 1].as_bytes()[i] {
                    throw = true;
                    cnt += 1;
                    break;
                }
            }
            if !throw {
                for j in 1..strs.len() {
                    if idx[j] == 0 && strs[j].as_bytes()[i] > strs[j - 1].as_bytes()[i] {
                        idx[j] = 1;
                        sorted += 1;
                    }
                }
                if sorted + 1 == n {
                    break;
                }
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_deletion_size() {
        assert_eq!(Solution::min_deletion_size(vec_str!["ca", "bb", "ac"]), 1);
        assert_eq!(Solution::min_deletion_size(vec_str!["xc", "yb", "za"]), 0);
        assert_eq!(Solution::min_deletion_size(vec_str!["zyx", "wvu", "tsr"]), 3);
    }
}
