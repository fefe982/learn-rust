// https://leetcode.com/problems/minimum-number-of-k-consecutive-bit-flips/
// 995. Minimum Number of K Consecutive Bit Flips
pub struct Solution;
impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let mut f = vec![false; nums.len()];
        let k = k as usize;
        let mut cnt = 0;
        let mut flips = 0;
        let l = nums.len();
        for (i, n) in nums.into_iter().enumerate() {
            if i >= k && f[i - k] {
                flips -= 1;
            }
            if flips % 2 == n {
                if i + k <= l {
                    flips += 1;
                    cnt += 1;
                    f[i] = true;
                } else {
                    return -1;
                }
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_k_bit_flips() {
        assert_eq!(Solution::min_k_bit_flips(vec![0, 1, 0], 1), 2);
        assert_eq!(Solution::min_k_bit_flips(vec![1, 1, 0], 2), -1);
        assert_eq!(Solution::min_k_bit_flips(vec![0, 0, 0, 1, 0, 1, 1, 0], 3), 3);
    }
}
