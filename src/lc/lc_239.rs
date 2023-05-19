// https://leetcode.com/problems/sliding-window-maximum/
// 239. Sliding Window Maximum
pub struct Solution;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut heap = std::collections::BinaryHeap::new();
        let mut removed = std::collections::BinaryHeap::new();
        let mut res = Vec::new();
        let k = k as usize;
        for idx in 0..k {
            heap.push(nums[idx]);
        }
        res.push(*heap.peek().unwrap());
        for idx in k..nums.len() {
            removed.push(nums[idx - k]);
            heap.push(nums[idx]);
            while !removed.is_empty() && removed.peek() == heap.peek() {
                removed.pop();
                heap.pop();
            }
            res.push(*heap.peek().unwrap())
        }
        res
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
