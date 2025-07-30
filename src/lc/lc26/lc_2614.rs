// https://leetcode.cn/problems/prime-in-diagonal/
// 2614. Prime In Diagonal
pub struct Solution;
impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let isprime = |x: i32| -> bool {
            if x <= 1 {
                return false;
            }
            let mut i = 2;
            while i * i <= x {
                if x % i == 0 {
                    return false;
                }
                i += 1;
            }
            true
        };
        let mut max = 0;
        for i in 0..nums.len() {
            if isprime(nums[i][i]) {
                max = max.max(nums[i][i]);
            }
            if isprime(nums[i][nums.len() - 1 - i]) {
                max = max.max(nums[i][nums.len() - 1 - i]);
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn diagonal_prime() {
        assert_eq!(
            Solution::diagonal_prime(vec_vec![[1, 2, 3], [5, 6, 7], [9, 10, 11]]),
            11
        );
        assert_eq!(
            Solution::diagonal_prime(vec_vec![[1, 2, 3], [5, 17, 7], [9, 11, 10]]),
            17
        );
    }
}
