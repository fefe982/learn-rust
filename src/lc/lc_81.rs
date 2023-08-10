// https://leetcode.com/problems/search-in-rotated-sorted-array-ii/
// 81. Search in Rotated Sorted Array II
pub struct Solution;
impl Solution {
    fn sch(nums: &[i32], target: i32) -> bool {
        let l = nums.len();
        if nums[0] == target || nums[l - 1] == target {
            true
        } else if l <= 2 {
            false
        } else {
            if nums[0] < nums[l - 1] {
                if let Ok(_) = nums.binary_search(&target) {
                    true
                } else {
                    false
                }
            } else if nums[0] > target && target > nums[l - 1] {
                false
            } else {
                let m = l / 2;
                if target == nums[m] {
                    true
                } else if nums[0] == nums[l - 1] && nums[0] == nums[m] {
                    Self::sch(&nums[1..nums.len() - 1], target)
                } else if target > nums[0] {
                    if target < nums[m] || nums[m] < nums[0] {
                        Self::sch(&nums[0..m], target)
                    } else {
                        Self::sch(&nums[m..], target)
                    }
                } else if target > nums[m] || nums[m] > nums[l - 1] {
                    Self::sch(&nums[m..], target)
                } else {
                    Self::sch(&nums[0..m], target)
                }
            }
        }
    }
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        Self::sch(&nums[..], target)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search() {
        assert_eq!(
            Solution::search(
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1],
                2
            ),
            true
        );
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
    }
}
