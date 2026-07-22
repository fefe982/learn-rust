// https://leetcode.com/problems/number-of-unique-xor-triplets-i/
// 3513. Number of Unique Good Triplets
pub struct Solution;
impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        if n <= 2 {
            return n;
        }
        let mut ans = 1;
        while ans <= n {
            ans *= 2;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unique_xor_triplets() {
        assert_eq!(Solution::unique_xor_triplets(vec![1, 2]), 2);
        assert_eq!(Solution::unique_xor_triplets(vec![3, 1, 2]), 4);
    }
}
