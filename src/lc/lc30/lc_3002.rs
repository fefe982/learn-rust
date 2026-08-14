// https://leetcode.com/problems/maximum-size-of-a-set-after-removals/
// 3002. Maximum Size of a Set After Removals
pub struct Solution;
impl Solution {
    pub fn maximum_set_size(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut m = std::collections::HashMap::new();
        for i in 0..nums1.len() {
            m.entry(nums1[i]).or_insert((0, 0)).0 = 1;
            m.entry(nums2[i]).or_insert((0, 0)).1 = 1;
        }
        let (mut n1, mut n2, n3) = m.into_iter().fold((0, 0, 0), |(n1, n2, n3), (_, (v1, v2))| {
            if v1 == 1 && v2 == 1 {
                (n1, n2, n3 + 1)
            } else if v1 == 1 {
                (n1 + 1, n2, n3)
            } else {
                (n1, n2 + 1, n3)
            }
        });
        let n = nums1.len() as i32;
        n1 = n1.min(n / 2);
        n2 = n2.min(n / 2);
        n1 + n2 + (n - (n1 + n2)).min(n3)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_set_size() {
        assert_eq!(Solution::maximum_set_size(vec![1, 2, 1, 2], vec![1, 1, 1, 1]), 2);
        assert_eq!(
            Solution::maximum_set_size(vec![1, 2, 3, 4, 5, 6], vec![2, 3, 2, 3, 2, 3]),
            5
        );
        assert_eq!(
            Solution::maximum_set_size(vec![1, 1, 2, 2, 3, 3], vec![4, 4, 5, 5, 6, 6]),
            6
        );
    }
}
