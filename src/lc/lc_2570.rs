// https://leetcode.com/problems/merge-two-2d-arrays-by-summing-values/
// 2570. Merge Two 2D Arrays by Summing Values
pub struct Solution;
impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut v = Vec::with_capacity(nums1.len() + nums2.len());
        let mut i = 0;
        let mut j = 0;
        loop {
            match (nums1.get(i), nums2.get(j)) {
                (Some(a), Some(b)) => match a[0].cmp(&b[0]) {
                    std::cmp::Ordering::Equal => {
                        v.push(vec![a[0], a[1] + b[1]]);
                        i += 1;
                        j += 1;
                    }
                    std::cmp::Ordering::Less => {
                        v.push(vec![a[0], a[1]]);
                        i += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        v.push(vec![b[0], b[1]]);
                        j += 1;
                    }
                },
                (Some(a), None) => {
                    v.push(vec![a[0], a[1]]);
                    i += 1;
                }
                (None, Some(b)) => {
                    v.push(vec![b[0], b[1]]);
                    j += 1;
                }
                (None, None) => break,
            }
        }
        v
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_merge_arrays() {
        assert_eq!(
            Solution::merge_arrays(vec_vec![[1, 2], [2, 3], [4, 5]], vec_vec![[1, 4], [3, 2], [4, 1]]),
            vec_vec![[1, 6], [2, 3], [3, 2], [4, 6]]
        );
        assert_eq!(
            Solution::merge_arrays(vec_vec![[2, 4], [3, 6], [5, 5]], vec_vec![[1, 3], [4, 3]]),
            vec_vec![[1, 3], [2, 4], [3, 6], [4, 3], [5, 5]]
        );
    }
}
