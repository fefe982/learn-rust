// https://leetcode.com/problems/split-array-into-maximum-number-of-subarrays/
// 2871. Split Array Into Maximum Number of Subarrays
pub struct Solution;
impl Solution {
    pub fn max_subarrays(nums: Vec<i32>) -> i32 {
        let and = nums.iter().fold(nums[0], |f, &x| f & x);
        if and > 0 {
            return 1;
        }
        let mut cnt = 0;
        let mut s = i32::MAX;
        for i in 0..nums.len() {
            s = s & nums[i];
            if s == 0 {
                cnt += 1;
                s = i32::MAX;
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_subarrays() {
        assert_eq!(Solution::max_subarrays(vec![22, 21, 29, 22]), 1);
        assert_eq!(Solution::max_subarrays(vec![1, 0, 2, 0, 1, 2]), 3);
        assert_eq!(Solution::max_subarrays(vec![5, 7, 1, 3]), 1);
    }
}
