// https://leetcode.com/problems/find-common-elements-between-two-arrays/
// 2956. Find Common Elements Between Two Arrays
pub struct Solution;
impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut m = std::collections::HashMap::<i32, i32>::new();
        for n in nums1 {
            *m.entry(n).or_default() += 1;
        }
        let mut v = vec![0, 0];
        for n in nums2 {
            if let Some(c) = m.get_mut(&n) {
                v[1] += 1;
                v[0] += *c;
                *c = 0;
            }
        }
        v
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_intersection_values() {
        assert_eq!(Solution::find_intersection_values(vec![2, 3, 2], vec![1, 2]), [2, 1]);
        assert_eq!(
            Solution::find_intersection_values(vec![4, 3, 2, 3, 1], vec![2, 2, 5, 2, 3, 6]),
            [3, 4]
        );
    }
}
