// https://leetcode.com/problems/minimum-array-length-after-pair-removals/
// 2856. Minimum Array Length After Removing Similar Pairs
pub struct Solution;
impl Solution {
    pub fn min_length_after_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut cnt = 0;
        let mut mcnt = 0;
        let mut last = 0;
        for i in 0..n {
            if nums[i] == last {
                cnt += 1;
            } else {
                mcnt = mcnt.max(cnt);
                cnt = 1;
                last = nums[i];
            }
        }
        mcnt = mcnt.max(cnt);
        if mcnt <= n / 2 {
            return (n % 2) as i32;
        }
        (mcnt * 2 - n) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_length_after_removals() {
        assert_eq!(Solution::min_length_after_removals(vec![2, 3, 4]), 1);
        assert_eq!(Solution::min_length_after_removals(vec![1, 2, 3, 4]), 0);
        assert_eq!(Solution::min_length_after_removals(vec![1, 1, 2, 2, 3, 3]), 0);
        assert_eq!(Solution::min_length_after_removals(vec![1000000000, 1000000000]), 2);
        assert_eq!(Solution::min_length_after_removals(vec![2, 3, 4, 4, 4]), 1);
    }
}
