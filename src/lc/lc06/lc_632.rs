// https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/
// 632. Smallest Range Covering Elements from K Lists
pub struct Solution;
impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut h = std::collections::BinaryHeap::new();
        let mut min_rng = i32::MAX;
        let mut rng = vec![];
        let mut max = i32::MIN;
        for i in 0..nums.len() {
            h.push((std::cmp::Reverse(nums[i][0]), i, 0usize));
            max = max.max(nums[i][0]);
        }
        loop {
            let (min, idx, pos) = h.pop().unwrap();
            if max - min.0 < min_rng {
                min_rng = max - min.0;
                rng = vec![min.0, max];
            }
            if pos + 1 >= nums[idx].len() {
                return rng;
            }
            h.push((std::cmp::Reverse(nums[idx][pos + 1]), idx, pos + 1));
            max = max.max(nums[idx][pos + 1]);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_smallest_range() {
        assert_eq!(
            Solution::smallest_range(vec_vec![
                [4, 10, 15, 24, 26],
                [0, 9, 12, 20],
                [5, 18, 22, 30]
            ]),
            vec![20, 24]
        );
        assert_eq!(
            Solution::smallest_range(vec_vec![[1, 2, 3], [1, 2, 3], [1, 2, 3]]),
            vec![1, 1]
        );
    }
}
