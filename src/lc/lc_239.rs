// https://leetcode.com/problems/sliding-window-maximum/
// 239. Sliding Window Maximum
pub struct Solution;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut q = std::collections::VecDeque::new();
        let mut v = Vec::with_capacity(nums.len() - k + 1);
        for i in 0..nums.len() {
            while q.back().is_some_and(|&j| nums[j] < nums[i]) {
                q.pop_back();
            }
            q.push_back(i);
            while q.front().is_some_and(|&j| j + k <= i) {
                q.pop_front();
            }
            if i >= k - 1 {
                v.push(nums[*q.front().unwrap()]);
            }
        }
        v
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_sliding_window() {
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
        assert_eq!(Solution::max_sliding_window(vec![1], 1), vec![1]);
    }
}
