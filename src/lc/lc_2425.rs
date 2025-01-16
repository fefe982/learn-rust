// https://leetcode.com/problems/bitwise-xor-of-all-pairings/
// 2425. Bitwise XOR of All Pairings
pub struct Solution;
impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        (if nums1.len() % 2 == 1 {
            nums2.iter().fold(0, |acc, &x| acc ^ x)
        } else {
            0
        }) ^ if nums2.len() % 2 == 1 {
            nums1.iter().fold(0, |acc, &x| acc ^ x)
        } else {
            0
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_xor_all_nums() {
        assert_eq!(Solution::xor_all_nums(vec![2, 1, 3], vec![10, 2, 5, 0]), 13);
        assert_eq!(Solution::xor_all_nums(vec![1, 2], vec![3, 4]), 0);
    }
}
