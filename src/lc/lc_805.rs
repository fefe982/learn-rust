// https://leetcode.com/problems/split-array-with-same-average/
// 805. Split Array With Same Average
pub struct Solution;
impl Solution {
    fn solve(
        nums: &[i32],
        target: i32,
        n: usize,
        cache: &mut std::collections::HashMap<(usize, i32, usize), bool>,
    ) -> bool {
        if n == 0 {
            return target == 0;
        }
        if target < 0 || n > nums.len() {
            return false;
        }
        if let Some(&v) = cache.get(&(nums.len(), target, n)) {
            return v;
        }
        let r = Self::solve(&nums[1..], target - nums[0], n - 1, cache) || Self::solve(&nums[1..], target, n, cache);
        cache.insert((nums.len(), target, n), r);
        r
    }
    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        let mut cache = std::collections::HashMap::new();
        for i in 1..=nums.len() / 2 {
            if sum as usize * i % nums.len() != 0 {
                continue;
            }
            if Self::solve(&nums, (sum as usize * i / nums.len()) as i32, i, &mut cache) {
                return true;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_split_array_same_average() {
        assert_eq!(Solution::split_array_same_average(vec![0, 0, 0, 0, 0]), true);
        assert_eq!(Solution::split_array_same_average(vec![5, 3, 11, 19, 2]), true);
        assert_eq!(Solution::split_array_same_average(vec![1, 2, 3, 4, 5, 6, 7, 8]), true);
        assert_eq!(Solution::split_array_same_average(vec![3, 1]), false);
    }
}
