// https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k/
// 862. Shortest Subarray with Sum at Least K
pub struct Solution;
impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = vec![0i64; nums.len() + 1];
        for i in 0..nums.len() {
            sum[i + 1] = sum[i] + nums[i] as i64;
        }
        let mut q = std::collections::VecDeque::new();
        q.push_back(0);
        let mut min = usize::MAX;
        for i in 1..=nums.len() {
            while let Some(&n) = q.back() {
                if sum[i] <= sum[n] {
                    q.pop_back();
                } else {
                    break;
                }
            }
            while let Some(&n) = q.front() {
                if sum[i] - sum[n] >= k as i64 {
                    min = std::cmp::min(min, i - n);
                    q.pop_front();
                } else {
                    break;
                }
            }
            q.push_back(i);
        }
        if min == usize::MAX {
            -1
        } else {
            min as i32
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shortest_subarray() {
        assert_eq!(Solution::shortest_subarray(vec![1], 1), 1);
        assert_eq!(Solution::shortest_subarray(vec![1, 2], 4), -1);
        assert_eq!(Solution::shortest_subarray(vec![2, -1, 2], 3), 3);
    }
}
