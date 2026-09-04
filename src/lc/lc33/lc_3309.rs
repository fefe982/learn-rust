// https://leetcode.com/problems/maximum-possible-number-by-binary-concatenation/
// 3309. Maximum Possible Number by Binary Concatenation
pub struct Solution;
impl Solution {
    pub fn max_good_number(nums: Vec<i32>) -> i32 {
        let cat = |a: i32, b: i32| -> i32 { a * (1 << (i32::BITS - b.leading_zeros())) + b };
        cat(nums[0], cat(nums[1], nums[2]))
            .max(cat(nums[0], cat(nums[2], nums[1])))
            .max(cat(nums[1], cat(nums[0], nums[2])))
            .max(cat(nums[1], cat(nums[2], nums[0])))
            .max(cat(nums[2], cat(nums[0], nums[1])))
            .max(cat(nums[2], cat(nums[1], nums[0])))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_good_number() {
        assert_eq!(Solution::max_good_number(vec![1, 2, 3]), 30);
        assert_eq!(Solution::max_good_number(vec![2, 8, 16]), 1296);
    }
}
