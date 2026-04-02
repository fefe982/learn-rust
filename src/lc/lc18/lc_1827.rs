// https://leetcode.com/problems/minimum-operations-to-make-the-array-increasing/
// 1827. Minimum Operations to Make the Array Increasing
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut prev = 0;
        for n in nums {
            if n <= prev {
                prev += 1;
                ans += prev - n;
            } else {
                prev = n;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations(vec![1, 1, 1]), 3);
        assert_eq!(Solution::min_operations(vec![1, 5, 2, 4, 1]), 14);
        assert_eq!(Solution::min_operations(vec![8]), 0);
    }
}
