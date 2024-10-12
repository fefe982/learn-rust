// https://leetcode.com/problems/find-the-xor-of-numbers-which-appear-twice/
// 3158. Find the XOR of Numbers Which Appear Twice
pub struct Solution;
impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut mask = 0i64;
        let mut res = 0;
        for n in nums {
            let m = 1 << n;
            if mask & m != 0 {
                res ^= n;
            } else {
                mask |= m;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_duplicate_numbers_xor() {
        assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 1, 3]), 1);
        assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 3]), 0);
        assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 2, 1]), 3);
    }
}
