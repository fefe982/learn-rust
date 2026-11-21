// https://leetcode.com/problems/minimum-set-bits-in-a-valid-integer/
// 4119. Minimum Set Bits in a Valid Integer
pub struct Solution;
impl Solution {
    pub fn min_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        let mx = *nums.iter().max().unwrap();
        let l = (i32::BITS - mx.leading_zeros()) as usize;
        let mask = (1 << l) - 1;
        let mut dp = vec![0; mask + 1];
        let mut c = vec![0; l];
        let k = k as usize;
        for i in 0..nums.len() {
            let mut n = nums[i];
            let mut v = 0;
            while n > 0 {
                let b = n.trailing_zeros() as usize;
                c[b] += 1;
                n ^= 1 << b;
                if c[b] == k {
                    v ^= 1 << b;
                }
            }
            if i >= k - 1 {
                if v == 0 {
                    return -1;
                }
                dp[mask ^ v] = 1;
                let mut n = nums[i + 1 - k];
                while n > 0 {
                    let b = n.trailing_zeros() as usize;
                    c[b] -= 1;
                    n ^= 1 << b;
                }
            }
        }
        for i in 0..l {
            let mut j = 1 << i;
            while j < dp.len() {
                if j & (1 << i) != 0 {
                    if dp[j] == 1 {
                        dp[j ^ (1 << i)] = 1;
                    }
                    j += 1;
                } else {
                    j += 1 << i;
                }
            }
        }
        let mut r = i32::MAX;
        for i in 1..dp.len() {
            if dp[i] == 0 {
                r = r.min(i.count_ones() as i32);
            }
        }
        if r == i32::MAX {
            -1
        } else {
            r
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_set_bits() {
        assert_eq!(Solution::min_set_bits(vec![6, 7, 3], 2), 1);
        assert_eq!(Solution::min_set_bits(vec![3, 5, 12], 1), 2);
        assert_eq!(Solution::min_set_bits(vec![2, 1], 2), -1);
    }
}
