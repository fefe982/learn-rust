// https://leetcode.com/problems/intersection-of-two-arrays/
// 349. Intersection of Two Arrays
pub struct Solution;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        std::collections::HashSet::<i32>::from_iter(nums1.into_iter())
            .intersection(&std::collections::HashSet::<i32>::from_iter(nums2.into_iter()))
            .cloned()
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_intersection() {
        assert_eq!(
            std::collections::HashSet::<i32>::from_iter(
                Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]).into_iter()
            ),
            std::collections::HashSet::from_iter(vec![2].into_iter())
        );
        assert_eq!(
            std::collections::HashSet::<i32>::from_iter(
                Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]).into_iter()
            ),
            std::collections::HashSet::from_iter(vec![4, 9].into_iter())
        );
    }
}
