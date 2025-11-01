// https://leetcode.com/problems/maximum-xor-of-subsequences/
// 3681. Maximum XOR of Subsequences
pub struct Solution;
impl Solution {
    pub fn max_xor_subsequences(nums: Vec<i32>) -> i32 {
        let mut basis = vec![0; 32];
        for mut n in nums {
            while n > 0 {
                let l = n.leading_zeros() as usize;
                if basis[l] == 0 {
                    basis[l] = n;
                    break;
                } else {
                    n ^= basis[l];
                }
            }
        }
        let mut res = 0;
        for b in basis {
            if res ^ b > res {
                res ^= b;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_xor_subsequences() {
        assert_eq!(Solution::max_xor_subsequences(vec![1, 2, 3]), 3);
        assert_eq!(Solution::max_xor_subsequences(vec![5, 2]), 7);
    }
}
