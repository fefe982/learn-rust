// https://leetcode.com/problems/find-xor-beauty-of-array/
// 2527. Find Xor Beauty of Array
pub struct Solution;
impl Solution {
    pub fn xor_beauty(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for &num in &nums {
            res ^= num;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_xor_beauty() {
        assert_eq!(Solution::xor_beauty(vec![1, 4]), 5);
        assert_eq!(Solution::xor_beauty(vec![15, 45, 20, 2, 34, 35, 5, 44, 32, 30]), 34);
    }
}
