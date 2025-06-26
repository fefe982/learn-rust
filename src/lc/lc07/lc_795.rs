// https://leetcode.com/problems/number-of-subarrays-with-bounded-maximum/
// 795. Number of Subarrays with Bounded Maximum
pub struct Solution;
impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let mut ans = 0;
        let mut ss = 1;
        let mut se = 0;
        for (num, i) in nums.into_iter().zip(1..) {
            if num > right {
                ss = i + 1;
                se = i;
            } else if num >= left {
                se = i;
            }
            ans += se + 1 - ss;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_subarray_bounded_max() {
        assert_eq!(
            Solution::num_subarray_bounded_max(vec![16, 69, 88, 85, 79, 87, 37, 33, 39, 34], 55, 57),
            0
        );
        assert_eq!(Solution::num_subarray_bounded_max(vec![2, 1, 4, 3], 2, 3), 3);
        assert_eq!(Solution::num_subarray_bounded_max(vec![2, 9, 2, 5, 6], 2, 8), 7);
    }
}
