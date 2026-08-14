// https://leetcode.com/problems/longest-subsequence-with-non-zero-bitwise-xor/
// 3702. Longest Subsequence With Limited Sum
pub struct Solution;
impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let mut x = 0;
        let mut nz = 0;
        let n = nums.len() as i32;
        for i in nums {
            x ^= i;
            if i != 0 {
                nz += 1;
            }
        }
        if x != 0 {
            n
        } else if nz != 0 {
            n - 1
        } else {
            0
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_subsequence() {
        assert_eq!(Solution::longest_subsequence(vec![1, 2, 3]), 2);
        assert_eq!(Solution::longest_subsequence(vec![2, 3, 4]), 3);
    }
}
