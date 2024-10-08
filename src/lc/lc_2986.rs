// https://leetcode.com/problems/apply-operations-to-maximize-frequency-score/
// 2986. Maximum Frequency Score of a Good Array
pub struct Solution;
impl Solution {
    pub fn max_frequency_score(nums: Vec<i32>, k: i64) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut i = 0;
        let mut j = 0;
        let mut max = 1;
        let mut acc = 0;
        loop {
            while j + 1 < nums.len() && acc <= k {
                max = max.max(j - i + 1);
                j += 1;
                acc += nums[j] as i64 - nums[(i + j) / 2] as i64;
            }
            if acc <= k {
                max = max.max(j - i + 1);
                break;
            }
            i += 1;
            acc -= nums[(i + j) / 2] as i64 - nums[i - 1] as i64;
        }
        max as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_frequency_score() {
        assert_eq!(Solution::max_frequency_score(vec![1, 2, 6, 4], 3), 3);
        assert_eq!(Solution::max_frequency_score(vec![1, 4, 4, 2, 4], 0), 3);
    }
}
