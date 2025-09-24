// https://leetcode.com/problems/decompress-run-length-encoded-list/
// 1313. Decompress Run-Length Encoded List
pub struct Solution;
impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        for i in 0..nums.len() / 2 {
            result.extend(std::iter::repeat(nums[2 * i + 1]).take(nums[2 * i] as usize));
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decompress_rl_elist() {
        assert_eq!(Solution::decompress_rl_elist(vec![1, 2, 3, 4]), vec![2, 4, 4, 4]);
        assert_eq!(Solution::decompress_rl_elist(vec![1, 1, 2, 3]), vec![1, 3, 3])
    }
}
