// https://leetcode.com/problems/range-sum-of-sorted-subarray-sums/
// 1508. Range Sum of Sorted Subarray Sums
pub struct Solution;
impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let mut h = std::collections::BinaryHeap::new();
        let mut seen = vec![vec![false; n as usize]; n as usize];
        for i in 0..n as usize {
            h.push((std::cmp::Reverse(nums[i]), i, i));
            seen[i][i] = true;
        }
        let mut sum = 0;
        for i in 0..right {
            if let Some((std::cmp::Reverse(v), s, e)) = h.pop() {
                if i + 1 >= left {
                    sum += v as i64;
                }
                if s > 0 && !seen[s - 1][e] {
                    h.push((std::cmp::Reverse(v + nums[s - 1]), s - 1, e));
                    seen[s - 1][e] = true;
                }
                if e < n as usize - 1 && !seen[s][e + 1] {
                    h.push((std::cmp::Reverse(v + nums[e + 1]), s, e + 1));
                    seen[s][e + 1] = true;
                }
            }
        }
        (sum % 1_0000_0000_7) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_range_sum() {
        assert_eq!(Solution::range_sum(vec![1, 2, 3, 4], 4, 1, 5), 13);
        assert_eq!(Solution::range_sum(vec![1, 2, 3, 4], 4, 3, 4), 6);
        assert_eq!(Solution::range_sum(vec![1, 2, 3, 4], 4, 1, 10), 50);
    }
}
