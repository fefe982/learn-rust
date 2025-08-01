// https://leetcode.com/problems/find-the-maximum-number-of-marked-indices/
// 2567. Find the Maximum Number of Marked Indices
pub struct Solution;
impl Solution {
    pub fn max_num_of_marked_indices(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        let mut nums = nums;
        nums.sort_unstable();
        let mut j = nums.len() - 1;
        let mut i = (nums.len() / 2) - 1;
        let mut ans = 0;
        loop {
            if nums[i] * 2 <= nums[j] {
                ans += 2;
                if i == 0 {
                    break;
                }
                i -= 1;
                j -= 1;
            } else if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_num_of_marked_indices() {
        assert_eq!(Solution::max_num_of_marked_indices(vec![36]), 0);
        assert_eq!(Solution::max_num_of_marked_indices(vec![3, 5, 2, 4]), 2);
        assert_eq!(Solution::max_num_of_marked_indices(vec![9, 2, 5, 4]), 4);
        assert_eq!(Solution::max_num_of_marked_indices(vec![7, 6, 8]), 0);
    }
}
