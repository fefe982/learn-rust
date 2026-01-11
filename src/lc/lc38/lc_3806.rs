// https://leetcode.com/problems/maximum-bitwise-and-after-increment-operations/
// 3806. Maximum Bitwise AND After Increment Operations
pub struct Solution;
impl Solution {
    pub fn maximum_and(nums: Vec<i32>, k: i32, m: i32) -> i32 {
        let mut ans = 0;
        for i in (0..31).rev() {
            let target = ans | (1 << i);
            let mut cnt = vec![0; nums.len()];
            for j in 0..nums.len() {
                let diff = target & !nums[j];
                let mask = ((1u32 << (i32::BITS - diff.leading_zeros())) - 1) as i32;
                cnt[j] = (target & mask) - (nums[j] & mask);
            }
            cnt.sort_unstable();
            if cnt
                .into_iter()
                .take(m as usize)
                .fold(0i32, |acc, x| acc.saturating_add(x))
                <= k
            {
                ans = target;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_and() {
        assert_eq!(Solution::maximum_and(vec![3, 1, 2], 8, 2), 6);
        assert_eq!(Solution::maximum_and(vec![1, 2, 8, 4], 7, 3), 4);
        assert_eq!(Solution::maximum_and(vec![1, 1], 3, 2), 2);
    }
}
