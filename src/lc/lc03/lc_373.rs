// https://leetcode.com/problems/find-k-pairs-with-smallest-sums/
// 373. Find K Pairs with Smallest Sums
pub struct Solution;
impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut q = std::collections::BinaryHeap::new();
        q.push((-nums1[0] - nums2[0], 0, 0));
        let mut res = vec![];
        let mut m = vec![usize::MAX; nums1.len()];
        m[0] = 0;
        for _ in 0..k {
            if let Some((_, i, j)) = q.pop() {
                res.push(vec![nums1[i], nums2[j]]);
                if i + 1 < nums1.len() && m[i + 1] == usize::MAX {
                    q.push((-nums1[i + 1] - nums2[0], i + 1, 0));
                    m[i + 1] = 0;
                }
                if j + 1 < nums2.len() && m[i] == j {
                    q.push((-nums1[i] - nums2[j + 1], i, j + 1));
                    m[i] = j + 1;
                }
            } else {
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
    fn k_smallest_pairs() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 10),
            vec_vec![
                [1, 1],
                [1, 1],
                [2, 1],
                [1, 2],
                [1, 2],
                [2, 2],
                [1, 3],
                [1, 3],
                [2, 3]
            ]
        );
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
            vec_vec![[1, 2], [1, 4], [1, 6]]
        );
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2),
            vec_vec![[1, 1], [1, 1]]
        );
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 2], vec![3], 3),
            vec_vec![[1, 3], [2, 3]]
        );
    }
}
