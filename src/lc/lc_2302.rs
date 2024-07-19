// https://leetcode.com/problems/count-subarrays-with-score-less-than-k/
// 2302. Count Subarrays With Score Less Than K
pub struct Solution;
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut res = 0i64;
        let mut sum = nums[0] as i64;
        let mut i = 0;
        let mut j = 0;
        loop {
            while j < nums.len() && sum * ((j - i + 1) as i64) < k {
                j += 1;
                if j < nums.len() {
                    sum += nums[j] as i64;
                }
            }
            res += (j - i) as i64;
            sum -= nums[i] as i64;
            i += 1;
            if i == nums.len() {
                return res;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_subarrays() {
        assert_eq!(Solution::count_subarrays(vec![2, 1, 4, 3, 5], 10), 6);
        assert_eq!(Solution::count_subarrays(vec![1, 1, 1], 5), 5);
    }
}
