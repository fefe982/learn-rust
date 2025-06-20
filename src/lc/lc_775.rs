// https://leetcode.com/problems/global-and-local-inversions/
// 775. Global and Local Inversions
pub struct Solution;
impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        let mut inv = false;
        for i in 0..nums.len() {
            if inv {
                if nums[i] != i as i32 - 1 {
                    return false;
                }
                inv = false;
            } else {
                if i as i32 + 1 == nums[i] {
                    inv = true;
                } else if i as i32 != nums[i] {
                    return false;
                }
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_ideal_permutation() {
        assert_eq!(Solution::is_ideal_permutation(vec![1, 0, 2]), true);
        assert_eq!(Solution::is_ideal_permutation(vec![1, 2, 0]), false);
    }
}
