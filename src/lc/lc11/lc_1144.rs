// https://leetcode.com/problems/decrease-elements-to-make-array-zigzag/
// 1144. Decrease Elements To Make Array Zigzag
pub struct Solution;
impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return 0;
        }
        let mut me = 0;
        let mut mo = 0;
        for i in 0..nums.len() {
            let m = if i == 0 {
                nums[i] - (nums[i + 1] - 1).min(nums[i])
            } else if i == nums.len() - 1 {
                nums[i] - (nums[i - 1] - 1).min(nums[i])
            } else {
                nums[i] - (nums[i - 1] - 1).min(nums[i + 1] - 1).min(nums[i])
            };
            if i % 2 == 0 {
                me += m;
            } else {
                mo += m;
            }
        }
        me.min(mo)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn moves_to_make_zigzag() {
        assert_eq!(Solution::moves_to_make_zigzag(vec![1, 2, 3]), 2);
        assert_eq!(Solution::moves_to_make_zigzag(vec![9, 6, 1, 6, 2]), 4);
    }
}
