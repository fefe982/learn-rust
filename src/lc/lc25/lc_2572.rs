// https://leetcode.com/problems/count-the-number-of-square-free-subsets/
// 2572. Count the Number of Square-Free Subsets
pub struct Solution;
const MOD: i64 = 1_000_000_007;
impl Solution {
    pub fn square_free_subsets(nums: Vec<i32>) -> i32 {
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        let mut pindx = vec![0; 31];
        for (i, &p) in primes.iter().enumerate() {
            pindx[p as usize] = i + 1;
        }
        let get_mask = |n: &i32| -> usize {
            let mut n = *n;
            let mut mask = 0;
            for &p in primes.iter() {
                let mut t = 0;
                while n % p == 0 {
                    t += 1;
                    n /= p;
                }
                if t == 1 {
                    mask |= 1 << pindx[p as usize];
                } else if t > 1 {
                    return 1;
                }
            }
            mask
        };
        let mut dp = vec![vec![0; 1 << 11]; nums.len()];
        let masks = nums.iter().map(get_mask).collect::<Vec<_>>();
        fn dfs(i: usize, mask: usize, masks: &Vec<usize>, dp: &mut Vec<Vec<i64>>) -> i64 {
            if i >= masks.len() {
                return 1;
            }
            if dp[i][mask] > 0 {
                return dp[i][mask];
            }
            let mut ans = dfs(i + 1, mask, masks, dp);
            if mask & masks[i] == 0 {
                ans = (ans + dfs(i + 1, mask | masks[i], masks, dp)) % MOD;
            }
            dp[i][mask] = ans;
            ans
        }
        ((dfs(0, 1, &masks, &mut dp) - 1 + MOD) % MOD) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn square_free_subsets() {
        assert_eq!(Solution::square_free_subsets(vec![3, 4, 4, 5]), 3);
        assert_eq!(Solution::square_free_subsets(vec![1]), 1);
    }
}
