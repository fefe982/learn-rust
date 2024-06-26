// https://leetcode.com/problems/special-permutations/
// 2741. Special Permutations
pub struct Solution;
impl Solution {
    pub fn special_perm(nums: Vec<i32>) -> i32 {
        let m = 1_0000_0000_7i64;
        let n = nums.len();
        let mut dp = vec![vec![0i64; n]; 1 << n];
        'state: for state in 1usize..(1 << n) {
            let mut ms = state;
            while ms > 0 {
                let bit = ms.trailing_zeros() as usize;
                let pstate = state ^ (1 << bit);
                if pstate == 0 {
                    dp[state][bit] = 1;
                    continue 'state;
                }
                let mut mps = pstate;
                while mps > 0 {
                    let pbit = mps.trailing_zeros() as usize;
                    if nums[bit] % nums[pbit] == 0 || nums[pbit] % nums[bit] == 0 {
                        dp[state][bit] += dp[pstate][pbit];
                    }
                    dp[state][bit] %= m;
                    mps = mps & (mps - 1);
                }
                ms = ms & (ms - 1);
            }
        }
        (dp[(1 << n) - 1].iter().sum::<i64>() % m) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_special_perm() {
        assert_eq!(Solution::special_perm(vec![2, 3, 6]), 2);
        assert_eq!(Solution::special_perm(vec![1, 4, 3]), 2);
    }
}
