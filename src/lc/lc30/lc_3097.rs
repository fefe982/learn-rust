// https://leetcode.com/problems/shortest-subarray-with-or-at-least-k-ii/
// 3097. Shortest Subarray With at Least K ORs
pub struct Solution;
impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        if k == 0 {
            return 1;
        }
        let mut cnt = vec![0; 31];
        let mut i = 0;
        let mut j = 0;
        let mut ans = usize::MAX;
        let mut sum = 0;
        loop {
            while j < nums.len() && sum < k {
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
            if sum < k {
                break;
            }
            while i < j && sum >= k {
                ans = ans.min(j - i);
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
        if ans == usize::MAX {
            -1
        } else {
            ans as i32
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_subarray_length() {
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2, 3], 2), 1);
        assert_eq!(Solution::minimum_subarray_length(vec![2, 1, 8], 10), 3);
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2], 0), 1);
    }
}
