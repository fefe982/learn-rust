// https://leetcode.com/problems/maximum-product-of-two-integers-with-no-common-bits/
// 3670. Maximum Product of Two Integers With No Common Bits
pub struct Solution;
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i64 {
        let max_n = nums.iter().max().unwrap();
        let max_bit = i32::BITS - max_n.leading_zeros();
        let mask = (1 << max_bit) - 1;
        let mut dp = vec![0; mask as usize + 1];
        for &n in &nums {
            dp[n as usize] = n;
        }
        for i in 3..=mask {
            if dp[i] != 0 || i.count_ones() == 1 {
                continue;
            }
            let mut ii = i;
            while ii > 0 {
                let j = 1 << ii.trailing_zeros();
                dp[i] = dp[i ^ j as usize].max(dp[i]);
                ii ^= j;
            }
        }
        let mut ans = 0;
        for n in nums {
            ans = ans.max(dp[mask ^ n as usize] as i64 * n as i64);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_product() {
        assert_eq!(Solution::max_product(vec![1, 2, 3, 4, 5, 6, 7]), 12);
        assert_eq!(Solution::max_product(vec![5, 6, 4]), 0);
        assert_eq!(Solution::max_product(vec![64, 8, 32]), 2048);
    }
}
