// https://leetcode.com/problems/keep-multiplying-found-values-by-two/
// 2154. Keep Multiplying Found Values by Two
pub struct Solution;
impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut i = 0;
        let mut ans = original;
        while i < nums.len() {
            if ans == nums[i] {
                ans *= 2;
            } else {
                i += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_final_value() {
        assert_eq!(Solution::find_final_value(vec![5, 3, 6, 1, 12], 3), 24);
        assert_eq!(Solution::find_final_value(vec![2, 7, 9], 4), 4);
    }
}
