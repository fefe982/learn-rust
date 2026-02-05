// https://leetcode.com/problems/maximum-product-of-subsequences-with-an-alternating-sum-equal-to-k/
// 3509. Maximum Product of Subsequences With an Alternating Sum Equal to K
pub struct Solution;
impl Solution {
    fn search(
        nums: &Vec<i32>,
        i: usize,
        sum: i32,
        prod: i32,
        add: bool,
        empty: bool,
        ans: &mut i32,
        k: i32,
        limit: i32,
        cache: &mut std::collections::HashSet<(usize, i32, i32, bool, bool)>,
    ) {
        if *ans == limit || (prod > limit && *ans >= 0) {
            return;
        }
        if i == nums.len() {
            if !empty && sum == k && prod <= limit {
                *ans = (*ans).max(prod);
            }
            return;
        }
        if cache.contains(&(i, sum, prod, add, empty)) {
            return;
        }
        Self::search(nums, i + 1, sum, prod, add, empty, ans, k, limit, cache);
        Self::search(
            nums,
            i + 1,
            if add { sum + nums[i] } else { sum - nums[i] },
            (prod * nums[i]).min(limit + 1),
            !add,
            false,
            ans,
            k,
            limit,
            cache,
        );
        cache.insert((i, sum, prod, add, empty));
    }
    pub fn max_product(nums: Vec<i32>, k: i32, limit: i32) -> i32 {
        let sum = nums.iter().sum::<i32>();
        if sum < k.abs() {
            return -1;
        }
        let mut ans = -1;
        let mut cache = std::collections::HashSet::new();
        Self::search(&nums, 0, 0, 1, true, true, &mut ans, k, limit, &mut cache);
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_product() {
        assert_eq!(Solution::max_product(vec![5, 0], 5, 20), 5);
        assert_eq!(Solution::max_product(vec![1, 2, 3], 2, 10), 6);
        assert_eq!(Solution::max_product(vec![2, 2, 3, 3], 0, 9), 9);
    }
}
