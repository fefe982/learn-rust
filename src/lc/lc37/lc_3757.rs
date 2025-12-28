// https://leetcode.com/problems/number-of-effective-subsequences/
// 3757. Number of Effective Subsequences
pub struct Solution;
impl Solution {
    pub fn count_effective(nums: Vec<i32>) -> i32 {
        let nums_or = nums.iter().fold(0, |acc, &x| acc | x);
        let bits = i32::BITS - nums_or.leading_zeros();
        let mut cnt = vec![0; 1 << bits];
        for &x in nums.iter() {
            cnt[x as usize] += 1;
        }
        for i in 0..bits {
            let i2 = 1 << i;
            let m = nums_or & !i2;
            if m == nums_or {
                continue;
            }
            let mut j = (!m + 1) & m;
            while j > 0 {
                cnt[(j | i2) as usize] += cnt[j as usize];
                j = ((!m | j) + 1) & m;
            }
        }
        const MOD: i64 = 1_000_000_007;
        let mut p2 = vec![1i64; nums.len() + 1];
        for i in 1..=nums.len() {
            p2[i] = (p2[i - 1] * 2) % MOD;
        }
        let mut j = 0;
        let mut ans = 0;
        while j < nums_or {
            if (nums_or ^ j).count_ones() & 1 == 0 {
                ans = (ans - p2[cnt[j as usize] as usize] + MOD) % MOD;
            } else {
                ans = (ans + p2[cnt[j as usize] as usize]) % MOD;
            }
            j = ((!nums_or | j) + 1) & nums_or;
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_effective() {
        assert_eq!(Solution::count_effective(vec![1, 2, 3]), 3);
        assert_eq!(Solution::count_effective(vec![7, 4, 6]), 4);
        assert_eq!(Solution::count_effective(vec![8, 8]), 1);
        assert_eq!(Solution::count_effective(vec![2, 2, 1]), 5);
    }
}
