// https://leetcode.com/problems/maximize-sum-of-at-most-k-distinct-elements/
// 3684. Maximize Sum of At Most K Distinct Elements
pub struct Solution;
impl Solution {
    pub fn max_k_distinct(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable_by_key(|x| std::cmp::Reverse(*x));
        let mut i = 1;
        let mut j = 1;
        let k = k as usize;
        while i < k && j < nums.len() {
            if nums[j] != nums[j - 1] {
                nums[i] = nums[j];
                i += 1;
                j += 1;
            } else {
                j += 1;
            }
        }
        nums.truncate(i);
        nums
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_k_distinct() {
        assert_eq!(
            Solution::max_k_distinct(vec![84, 93, 100, 77, 90], 3),
            vec![100, 93, 90]
        );
        assert_eq!(
            Solution::max_k_distinct(vec![84, 93, 100, 77, 93], 3),
            vec![100, 93, 84]
        );
        assert_eq!(Solution::max_k_distinct(vec![1, 1, 1, 2, 2, 2], 6), vec![2, 1]);
    }
}
