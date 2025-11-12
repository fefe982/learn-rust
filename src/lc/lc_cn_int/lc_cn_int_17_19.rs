// https://leetcode.com/problems/missing-two-lcci/
// 面试题 17.19. Missing Two LCCI
pub struct Solution;
impl Solution {
    pub fn missing_two(nums: Vec<i32>) -> Vec<i32> {
        let mut xor0 = 0;
        let len = nums.len() as i32;
        for i in 0..nums.len() {
            xor0 ^= nums[i];
            xor0 ^= (i + 1) as i32;
        }
        xor0 ^= len + 1;
        xor0 ^= len + 2;
        let m = 1 << xor0.trailing_zeros();
        let mut xor1 = 0;
        xor0 = 0;
        for i in 0..nums.len() {
            if nums[i] & m == 0 {
                xor0 ^= nums[i];
            } else {
                xor1 ^= nums[i];
            }
            if (i as i32 + 1) & m == 0 {
                xor0 ^= (i + 1) as i32;
            } else {
                xor1 ^= (i + 1) as i32;
            }
        }
        if (len + 1) & m == 0 {
            xor0 ^= len + 1;
        } else {
            xor1 ^= len + 1;
        }
        if (len + 2) & m == 0 {
            xor0 ^= len + 2;
        } else {
            xor1 ^= len + 2;
        }
        vec![xor0, xor1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn missing_two() {
        assert_sort_eq!(Solution::missing_two(vec![1]), vec![2, 3]);
        assert_sort_eq!(Solution::missing_two(vec![2, 3]), vec![1, 4]);
    }
}
