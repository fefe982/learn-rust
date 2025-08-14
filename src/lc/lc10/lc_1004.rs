// https://leetcode.com/problems/max-consecutive-ones-iii/
// 1004. Max Consecutive Ones III
pub struct Solution;
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut c = 0;
        let mut max = 0;
        for right in 0..nums.len() {
            if nums[right] == 0 {
                c += 1;
            }
            while c > k {
                if nums[left] == 0 {
                    c -= 1;
                }
                left += 1;
            }
            max = max.max(right - left + 1);
        }
        max as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_ones() {
        assert_eq!(Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2), 6);
        assert_eq!(
            Solution::longest_ones(vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1], 3),
            10
        );
    }
}
