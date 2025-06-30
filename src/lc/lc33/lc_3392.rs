// https://leetcode.com/problems/count-subarrays-of-length-three-with-a-condition/
// 3392. Count Subarrays of Length Three With a Condition
pub struct Solution;
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        for i in 2..nums.len() {
            if (nums[i - 2] + nums[i]) * 2 == nums[i - 1] {
                cnt += 1;
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_subarrays() {
        assert_eq!(Solution::count_subarrays(vec![1, 2, 1, 4, 1]), 1);
        assert_eq!(Solution::count_subarrays(vec![1, 1, 1]), 0);
    }
}
