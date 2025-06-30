// https://leetcode.com/problems/find-the-number-of-subsequences-with-equal-gcd/
// 3336. Find the Number of Subsequences With Equal GCD
pub struct Solution;
impl Solution {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        loop {
            if a == 0 {
                return b;
            }
            b %= a;
            if b == 0 {
                return a;
            }
            a %= b;
        }
    }
    fn search(nums: &Vec<i32>, i: usize, gcd1: i32, gcd2: i32, save: &mut Vec<[[i64; 201]; 201]>) -> i64 {
        if i == nums.len() {
            if gcd2 != 0 && gcd1 == gcd2 {
                return 1;
            } else {
                return 0;
            }
        }
        if save[i][gcd1 as usize][gcd2 as usize] != 0 {
            return save[i][gcd1 as usize][gcd2 as usize];
        }
        let ret = (Self::search(nums, i + 1, Self::gcd(gcd1, nums[i]), gcd2, save)
            + Self::search(nums, i + 1, gcd1, Self::gcd(gcd2, nums[i]), save)
            + Self::search(nums, i + 1, gcd1, gcd2, save))
            % 1000000007;
        save[i][gcd1 as usize][gcd2 as usize] = ret;
        ret
    }
    pub fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
        Self::search(&nums, 0, 0, 0, &mut vec![[[0; 201]; 201]; nums.len()]) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_subsequence_pair_count() {
        assert_eq!(Solution::subsequence_pair_count(vec![1, 2, 3, 4]), 10);
        assert_eq!(Solution::subsequence_pair_count(vec![10, 20, 30]), 2);
        assert_eq!(Solution::subsequence_pair_count(vec![1, 1, 1, 1]), 50);
    }
}
