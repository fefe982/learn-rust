// https://leetcode.com/problems/jump-game-vi/
// 1696. Jump Game VI
pub struct Solution;
impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let mut q = std::collections::BinaryHeap::new();
        q.push((nums[0], 0));
        let mut max = nums[0];
        for (i, n) in nums.into_iter().enumerate().skip(1) {
            while let Some((v, idx)) = q.peek() {
                if i - *idx > k as usize {
                    q.pop();
                } else {
                    max = v + n;
                    q.push((max, i));
                    break;
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
    fn test_max_results() {
        assert_eq!(Solution::max_result(vec![1, -1, -2, 4, -7, 3], 2), 7);
        assert_eq!(Solution::max_result(vec![10, -5, -2, 4, 0, 3], 3), 17);
        assert_eq!(Solution::max_result(vec![1, -5, -20, 4, -1, 3, -6, -3], 2), 0);
    }
}
