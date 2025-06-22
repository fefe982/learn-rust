// https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/
// 2044. Count Number of Maximum Bitwise-OR Subsets
pub struct Solution;
impl Solution {
    fn count(
        nums: &Vec<i32>,
        ornums: &Vec<i32>,
        idx: usize,
        mask: i32,
        cache: &mut std::collections::HashMap<(i32, usize), i32>,
    ) -> i32 {
        if mask == 0 {
            return 2 << idx;
        }
        if mask & ornums[idx] < mask {
            return 0;
        }
        if idx == 0 {
            return 1;
        }
        if let Some(v) = cache.get(&(mask, idx)) {
            return *v;
        }
        let res = Self::count(nums, ornums, idx - 1, mask, cache)
            + Self::count(nums, ornums, idx - 1, mask & !nums[idx], cache);
        cache.insert((mask, idx), res);
        res
    }
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut ornums = nums.clone();
        for i in 1..ornums.len() {
            ornums[i] |= ornums[i - 1];
        }
        let mask = ornums[ornums.len() - 1];
        Self::count(
            &nums,
            &ornums,
            ornums.len() - 1,
            mask,
            &mut std::collections::HashMap::new(),
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_max_or_subsets() {
        assert_eq!(Solution::count_max_or_subsets(vec![3, 1]), 2);
        assert_eq!(Solution::count_max_or_subsets(vec![2, 2, 2]), 7);
        assert_eq!(Solution::count_max_or_subsets(vec![3, 2, 1, 5]), 6);
    }
}
