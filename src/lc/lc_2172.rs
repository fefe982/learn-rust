// https://leetcode.com/problems/maximum-and-sum-of-array/
// 2172. Maximum AND Sum of Array
pub struct Solution;
impl Solution {
    pub fn maximum_and_sum(nums: Vec<i32>, num_slots: i32) -> i32 {
        let mut nums = nums;
        let nbit = num_slots as usize * 2;
        nums.resize(nbit, 0);
        let mut dp = vec![0; 1 << nbit];
        for i in 1usize..(1 << nbit) {
            let cnt = i.count_ones() as i32;
            let slot = (cnt + 1) / 2;
            for bit in 0..nbit {
                if i & (1 << bit) != 0 {
                    dp[i] = dp[i].max(dp[i ^ (1 << bit)] + (slot & nums[bit]));
                }
            }
        }
        dp[(1 << nbit) - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_and_sum() {
        assert_eq!(Solution::maximum_and_sum(vec![1, 2, 3, 4, 5, 6], 3), 9);
        assert_eq!(Solution::maximum_and_sum(vec![1, 3, 10, 4, 7, 1], 9), 24);
    }
}
