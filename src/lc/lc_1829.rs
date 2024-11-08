// https://leetcode.com/problems/maximum-xor-for-each-query/
// 1829. Maximum XOR for Each Query
pub struct Solution;
impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let len = nums.len();
        let mut res = vec![0; len];
        let mut xor = (1 << maximum_bit) - 1;
        for (i, n) in nums.into_iter().enumerate() {
            xor ^= n;
            res[len - i - 1] = xor;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_maximum_xor() {
        assert_eq!(Solution::get_maximum_xor(vec![0, 1, 1, 3], 2), [0, 3, 2, 3]);
        assert_eq!(Solution::get_maximum_xor(vec![2, 3, 4, 7], 3), [5, 2, 6, 5]);
        assert_eq!(Solution::get_maximum_xor(vec![0, 1, 2, 2, 5, 7], 3), [4, 3, 6, 4, 6, 7]);
    }
}
