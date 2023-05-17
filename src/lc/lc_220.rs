// https://leetcode.com/problems/contains-duplicate-iii/description/
// 220. Contains Duplicate III
pub struct Solution;
impl Solution {
    pub fn contains_nearby_almost_duplicate(
        mut nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        let mut buckets = std::collections::HashMap::<i32, i32>::new();
        let index_diff = index_diff as usize;
        for idx in 0..nums.len() {
            let n = nums[idx];
            let b = n / (value_diff + 1) - if n < 0 { 1 } else { 0 };
            nums[idx] = b;
            if buckets.contains_key(&b) {
                return true;
            }
            if let Some(&i) = buckets.get(&(b - 1)) {
                if n - i <= value_diff {
                    return true;
                }
            }
            if let Some(&i) = buckets.get(&(b + 1)) {
                if i - n <= value_diff {
                    return true;
                }
            }
            buckets.insert(b, n);
            if idx >= index_diff {
                buckets.remove(&(nums[idx - index_diff]));
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn contains_nearby_almost_duplicate() {
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0),
            true
        );
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3),
            false
        );
    }
}
