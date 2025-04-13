// https://leetcode.com/problems/4sum-ii/
// 454. 4Sum II
pub struct Solution;
impl Solution {
    pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for i in nums1 {
            for &j in &nums2 {
                *map.entry(i + j).or_insert(0) += 1;
            }
        }
        let mut ans = 0;
        for i in nums3 {
            for &j in &nums4 {
                if let Some(v) = map.get(&-(i + j)) {
                    ans += v;
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_four_sum_count() {
        assert_eq!(
            Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
            2
        );
        assert_eq!(Solution::four_sum_count(vec![0], vec![0], vec![0], vec![0]), 1);
    }
}
