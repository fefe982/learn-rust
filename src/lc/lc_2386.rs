// https://leetcode.com/problems/find-the-k-sum-of-an-array/
// 2386. Find the K-Sum of an Array
pub struct Solution;
impl Solution {
    pub fn k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums;
        let mut sum = 0;
        for n in nums.iter_mut() {
            if *n > 0 {
                sum += *n as i64;
            } else if *n < 0 {
                *n = -*n;
            }
        }
        if k == 1 {
            return sum;
        }
        let mut k = k;
        k -= 1;
        nums.sort_unstable();
        let mut q = std::collections::BinaryHeap::<std::cmp::Reverse<(i64, usize)>>::new();
        q.push(std::cmp::Reverse((nums[0] as i64, 0)));
        while let Some(std::cmp::Reverse((s, n))) = q.pop() {
            k -= 1;
            if k == 0 {
                return sum - s;
            }
            if n + 1 < nums.len() {
                q.push(std::cmp::Reverse((s - nums[n] as i64 + nums[n + 1] as i64, n + 1)));
                q.push(std::cmp::Reverse((s + nums[n + 1] as i64, n + 1)));
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_k_sum() {
        assert_eq!(Solution::k_sum(vec![2, 4, -2], 5), 2);
        assert_eq!(Solution::k_sum(vec![1, -2, 3, 4, -10, 12], 16), 10);
    }
}
