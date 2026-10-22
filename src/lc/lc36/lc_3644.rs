// https://leetcode.com/problems/maximum-k-to-sort-a-permutation/
// 3644. Maximum K to Sort a Permutation
pub struct Solution;
impl Solution {
    pub fn sort_permutation(nums: Vec<i32>) -> i32 {
        let mut and = i32::MAX;
        for i in 0..nums.len() {
            if i as i32 != nums[i] {
                and &= nums[i];
            }
        }
        if and == i32::MAX {
            0
        } else {
            and
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sort_permutation() {
        assert_eq!(Solution::sort_permutation(vec![0, 3, 2, 1]), 1);
        assert_eq!(Solution::sort_permutation(vec![0, 1, 3, 2]), 2);
        assert_eq!(Solution::sort_permutation(vec![3, 2, 1, 0]), 0);
    }
}
