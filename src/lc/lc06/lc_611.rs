// https://leetcode.com/problems/valid-triangle-number/
// 611. Valid Triangle Number
pub struct Solution;
impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut res = 0;
        for k in 2..nums.len() {
            let mut i = 0;
            let mut j = k - 1;
            while i < j {
                while i < j && nums[i] + nums[j] > nums[k] {
                    j -= 1;
                }
                res += (k - j - 1) as i32;
                i += 1;
            }
            res += ((k - i - 1) * (k - i) / 2) as i32
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn triangle_number() {
        assert_eq!(Solution::triangle_number(vec![2, 2, 3, 4]), 3);
        assert_eq!(Solution::triangle_number(vec![4, 2, 3, 4]), 4);
    }
}
