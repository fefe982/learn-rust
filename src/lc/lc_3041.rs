// https://leetcode.com/problems/maximize-consecutive-elements-in-an-array-after-modification/
// 3041. Maximize Consecutive Elements in an Array After Modification
pub struct Solution;
impl Solution {
    fn count(nums: &Vec<i32>, len: &mut Vec<Vec<i32>>, i: usize, mode: usize) -> i32 {
        if i >= nums.len() {
            return 0;
        }
        if i + 1 == nums.len() {
            return 1;
        }
        if len[i][mode] != -1 {
            return len[i][mode];
        }
        let mut c = 1;
        if mode == 0 {
            if nums[i] == nums[i + 1] {
                c = 1 + Self::count(nums, len, i + 1, 1);
            } else if nums[i] + 1 == nums[i + 1] {
                c = 1 + Self::count(nums, len, i + 1, 0);
            }
        } else {
            if nums[i] == nums[i + 1] {
                c = Self::count(nums, len, i + 1, 1);
            } else if nums[i] + 1 == nums[i + 1] {
                c = 1 + Self::count(nums, len, i + 1, 1);
            } else if nums[i] + 2 == nums[i + 1] {
                c = 1 + Self::count(nums, len, i + 1, 0);
            }
        }
        len[i][mode] = c;
        c
    }
    pub fn max_selected_elements(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut i = 2;
        let mut j = 2;
        while j < nums.len() {
            while j < nums.len() && nums[j] == nums[i - 2] {
                j += 1;
            }
            if j >= nums.len() {
                break;
            }
            nums[i] = nums[j];
            i += 1;
            j += 1;
        }
        i = i.min(nums.len());
        nums.truncate(i);
        let mut len = vec![vec![-1; 2]; nums.len()];
        let mut res = 0;
        for i in 0..nums.len() {
            res = res.max(Self::count(&nums, &mut len, i, 0));
            res = res.max(Self::count(&nums, &mut len, i, 1));
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_selected_elements() {
        assert_eq!(
            Solution::max_selected_elements(vec![8, 11, 19, 19, 1, 16, 13, 8, 15, 19, 9, 19, 2, 9, 10]),
            6
        );
        assert_eq!(
            Solution::max_selected_elements(vec![8, 10, 6, 12, 9, 12, 2, 3, 13, 19, 11, 18, 10, 16]),
            8
        );
        assert_eq!(Solution::max_selected_elements(vec![12, 11, 8, 7, 2, 10, 18, 12]), 6);
        assert_eq!(Solution::max_selected_elements(vec![2, 1, 5, 1, 1]), 3);
        assert_eq!(Solution::max_selected_elements(vec![1, 4, 7, 10]), 1);
    }
}
