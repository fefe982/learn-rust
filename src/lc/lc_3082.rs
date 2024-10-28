// https://leetcode.com/problems/find-the-sum-of-the-power-of-all-subsequences/
// 3082. Find the Sum of the Power of All Subsequences
pub struct Solution;
const MOD: i64 = 1_0000_0000_7;
impl Solution {
    fn pow2(mut n: usize, p2: &mut Vec<i64>) -> i64 {
        if p2[n] != 0 {
            return p2[n];
        }
        let mut b = 2;
        let mut res = 1;
        while n > 0 {
            if n & 1 == 1 {
                res = (res * b) % MOD;
            }
            b = (b * b) % MOD;
            n >>= 1;
        }
        p2[n] = res;
        res
    }
    fn count(
        nums: &Vec<i32>,
        sum: i32,
        k: i32,
        i: usize,
        len: usize,
        save: &mut Vec<Vec<Vec<i64>>>,
        p2: &mut Vec<i64>,
    ) -> i64 {
        if k == sum {
            return Self::pow2(nums.len() - len, p2);
        }
        if i == nums.len() || sum > k {
            return 0;
        }
        if save[i][len][sum as usize] != -1 {
            return save[i][len][sum as usize];
        }
        let res = (Self::count(nums, sum, k, i + 1, len, save, p2)
            + Self::count(nums, sum + nums[i], k, i + 1, len + 1, save, p2))
            % MOD;
        save[i][len][sum as usize] = res;
        res
    }
    pub fn sum_of_power(nums: Vec<i32>, k: i32) -> i32 {
        let mut p2 = vec![0; nums.len()];
        let mut save = vec![vec![vec![-1; k as usize + 1]; nums.len() + 1]; nums.len()];
        Self::count(&nums, 0, k, 0, 0, &mut save, &mut p2) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_of_power() {
        assert_eq!(Solution::sum_of_power(vec![1, 2, 3], 3), 6);
        assert_eq!(Solution::sum_of_power(vec![2, 3, 3], 5), 4);
        assert_eq!(Solution::sum_of_power(vec![1, 2, 3], 7), 0);
    }
}
