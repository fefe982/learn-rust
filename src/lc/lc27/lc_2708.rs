// https://leetcode.com/problems/maximum-strength-of-a-group/
// 2708. Maximum Strength of a Group
pub struct Solution;
impl Solution {
    pub fn max_strength(nums: Vec<i32>) -> i64 {
        let mut prod = 1;
        let mut neg = i32::MIN;
        let mut nz = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nz += 1;
                prod *= nums[i] as i64;
                if nums[i] < 0 {
                    neg = neg.max(nums[i]);
                }
            }
        }
        if nz == 0 {
            0
        } else if prod < 0 {
            if nz == 1 {
                if nums.len() > 1 {
                    0
                } else {
                    prod
                }
            } else {
                prod / neg as i64
            }
        } else {
            prod
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_strength() {
        assert_eq!(Solution::max_strength(vec![0, -1]), 0);
        assert_eq!(Solution::max_strength(vec![-9]), -9);
        assert_eq!(Solution::max_strength(vec![3, -1, -5, 2, 5, -9]), 1350);
        assert_eq!(Solution::max_strength(vec![-4, -5, -4]), 20);
    }
}
