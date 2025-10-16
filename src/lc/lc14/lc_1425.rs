// https://leetcode.com/problems/constrained-subsequence-sum/
// 1425. Constrained Subsequence Sum
pub struct Solution;
impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut h = std::collections::BinaryHeap::new();
        let mut max = nums[0];
        let k = k as usize;
        h.push((nums[0], 0));
        for i in 1..nums.len() {
            while let Some(&(s, idx)) = h.peek() {
                if i - idx > k {
                    h.pop();
                    continue;
                }
                let ss = s.max(0) + nums[i];
                max = max.max(ss);
                h.push((ss, i));
                break;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constrained_subset_sum() {
        assert_eq!(
            Solution::constrained_subset_sum(vec![10, 2, -10, 5, 20], 2),
            37
        );
        assert_eq!(Solution::constrained_subset_sum(vec![-1, -2, -3], 1), -1);
        assert_eq!(
            Solution::constrained_subset_sum(vec![10, -2, -10, -5, 20], 2),
            23
        );
    }
}
