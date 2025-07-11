// https://leetcode.com/problems/maximum-sum-of-distinct-subarrays-with-length-k/
// 2461. Maximum Sum of Distinct Subarrays With Length K
pub struct Solution;
impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut sum = 0;
        let mut max = 0;
        let mut set = vec![false; 100001];
        let mut s = 0;
        for e in 0..nums.len() {
            if set[nums[e] as usize] {
                while nums[s] != nums[e] {
                    set[nums[s] as usize] = false;
                    sum -= nums[s] as i64;
                    s += 1;
                }
                s += 1;
            } else {
                set[nums[e] as usize] = true;
                sum += nums[e] as i64;
                if (e - s + 1) as i32 == k {
                    max = max.max(sum);
                    set[nums[s] as usize] = false;
                    sum -= nums[s] as i64;
                    s += 1;
                }
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_subarray_sum() {
        assert_eq!(Solution::maximum_subarray_sum(vec![1, 5, 4, 2, 9, 9, 9], 3), 15);
        assert_eq!(Solution::maximum_subarray_sum(vec![4, 4, 4], 3), 0);
    }
}
