// https://leetcode.com/problems/minimum-numbers-of-function-calls-to-make-target-array/
// 1558. Minimum Numbers of Function Calls to Make Target Array
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut min_bit = i32::BITS;
        let mut ans = 0;
        for num in nums {
            min_bit = min_bit.min(num.leading_zeros());
            ans += num.count_ones();
        }
        if ans == 0 {
            0
        } else {
            (ans + i32::BITS - min_bit - 1) as i32
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations(vec![0]), 0);
        assert_eq!(Solution::min_operations(vec![1, 5]), 5);
        assert_eq!(Solution::min_operations(vec![2, 2]), 3);
        assert_eq!(Solution::min_operations(vec![4, 2, 5]), 6);
    }
}
