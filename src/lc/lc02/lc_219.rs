// https://leetcode.com/problems/contains-duplicate-ii/
// 219. Contains Duplicate II
pub struct Solution;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut s = std::collections::HashSet::new();
        for (i, &n) in nums.iter().enumerate() {
            if s.contains(&n) {
                return true;
            }
            s.insert(n);
            if i >= k as usize {
                s.remove(&nums[i - k as usize]);
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_contains_nearby_duplicate() {
        assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3), true);
        assert_eq!(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1), true);
        assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2), false);
    }
}
