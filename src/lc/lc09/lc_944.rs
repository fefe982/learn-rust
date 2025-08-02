// https://leetcode.com/problems/delete-columns-to-make-sorted/
// 944. Delete Columns to Make Sorted
pub struct Solution;
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut filtered = vec![false; strs[0].len()];
        for i in 1..strs.len() {
            let bl = strs[i - 1].as_bytes();
            let bi = strs[i].as_bytes();
            for j in 0..bl.len() {
                if bl[j] > bi[j] {
                    filtered[j] = true;
                }
            }
        }
        filtered.into_iter().filter(|x| *x).count() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_deletion_size() {
        assert_eq!(Solution::min_deletion_size(vec_str!["cba", "daf", "ghi"]), 1);
        assert_eq!(Solution::min_deletion_size(vec_str!["a", "b"]), 0);
        assert_eq!(Solution::min_deletion_size(vec_str!["zyx", "wvu", "tsr"]), 3);
    }
}
