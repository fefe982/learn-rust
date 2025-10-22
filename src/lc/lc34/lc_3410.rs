// https://leetcode.com/problems/maximize-subarray-sum-after-removing-all-occurrences-of-one-element/
// 3410. Maximize Subarray Sum After Removing All Occurrences of One Element
pub struct Solution;
impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>) -> i64 {
        let mut max = nums[0] as i64;
        let mut prefix = 0;
        let mut pre = std::collections::HashMap::new();
        let mut p0 = 0;
        let mut low = 0;
        for n in nums {
            let n = n as i64;
            prefix += n as i64;
            max = max.max(prefix - low);
            if n < 0 {
                let pn = *pre.get(&n).unwrap_or(&0);
                let npn = pn.min(p0) + n;
                pre.insert(n, npn);
                low = low.min(npn);
            }
            p0 = p0.min(prefix);
            low = low.min(p0);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_subarray_sum() {
        assert_eq!(Solution::max_subarray_sum(vec![-2, -2, -2]), -2);
        assert_eq!(Solution::max_subarray_sum(vec![-3, 2, -2, -1, 3, -2, 3]), 7);
        assert_eq!(Solution::max_subarray_sum(vec![1, 2, 3, 4]), 10);
    }
}
