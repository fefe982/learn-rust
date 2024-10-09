// https://leetcode.com/problems/find-subarray-with-bitwise-or-closest-to-k/
// 3171. Find Subarray With Bitwise OR Closest to K
pub struct Solution;
impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = vec![0; 31];
        let mut i = 0;
        let mut j = 0;
        let mut ans = i32::MAX;
        let mut sum = 0;
        loop {
            while j < nums.len() && sum <= k {
                if i != j {
                    ans = ans.min(k - sum);
                }
                let n = nums[j];
                let mut bit = 1;
                for i in 0..31 {
                    if n & bit != 0 {
                        cnt[i] += 1;
                        sum |= bit;
                    }
                    bit <<= 1;
                }
                j += 1;
            }
            if sum <= k {
                if i != j {
                    ans = ans.min(k - sum);
                }
                break;
            }
            while i < j && sum > k {
                ans = ans.min(sum - k);
                let n = nums[i];
                let mut bit = 1;
                for i in 0..31 {
                    if n & bit != 0 {
                        cnt[i] -= 1;
                        if cnt[i] == 0 {
                            sum ^= bit;
                        }
                    }
                    bit <<= 1;
                }
                i += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_difference() {
        assert_eq!(Solution::minimum_difference(vec![1, 2, 4, 5], 3), 0);
        assert_eq!(Solution::minimum_difference(vec![1, 3, 1, 3], 2), 1);
        assert_eq!(Solution::minimum_difference(vec![1], 10), 9);
    }
}
