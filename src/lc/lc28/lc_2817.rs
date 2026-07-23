// https://leetcode.com/problems/minimum-absolute-difference-between-elements-with-constraint/
// 2817. Minimum Absolute Difference Between Elements With Constraint
pub struct Solution;
impl Solution {
    pub fn min_absolute_difference(nums: Vec<i32>, x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let mut s = std::collections::BTreeSet::new();
        let x = x as usize;
        let mut ans = i32::MAX;
        for i in 0..nums.len() - x {
            s.insert(nums[i]);
            let v = nums[i + x];
            if let Some(&v1) = s.range(..=v).last() {
                ans = ans.min(v - v1);
            }
            if let Some(&v1) = s.range(v..).next() {
                ans = ans.min(v1 - v);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_absolute_difference() {
        assert_eq!(Solution::min_absolute_difference(vec![4, 3, 2, 4], 2), 0);
        assert_eq!(Solution::min_absolute_difference(vec![5, 3, 2, 10, 15], 1), 1);
        assert_eq!(Solution::min_absolute_difference(vec![1, 2, 3, 4], 3), 3);
    }
}
