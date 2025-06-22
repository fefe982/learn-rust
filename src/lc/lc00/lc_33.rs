// https://leetcode.com/problems/search-in-rotated-sorted-array/
// 33. Search in Rotated Sorted Array
pub struct Solution;
impl Solution {
    fn sch(nums: &[i32], target: i32) -> i32 {
        let l = nums.len();
        if nums[0] == target {
            0
        } else if nums[l - 1] == target {
            l as i32 - 1
        } else if l <= 2 {
            -1
        } else {
            if nums[0] < nums[l - 1] {
                if let Ok(i) = nums.binary_search(&target) {
                    i as i32
                } else {
                    -1
                }
            } else if nums[0] > target && target > nums[l - 1] {
                -1
            } else {
                let m = l / 2;
                if target == nums[m] {
                    m as i32
                } else if target >= nums[0] {
                    if target <= nums[m] || nums[m] < nums[0] {
                        Self::sch(&nums[0..m], target)
                    } else {
                        let idx = Self::sch(&nums[m..], target);
                        if idx > 0 {
                            idx + m as i32
                        } else {
                            -1
                        }
                    }
                } else if target >= nums[m] || nums[m] > nums[l - 1] {
                    let idx = Self::sch(&nums[m..], target);
                    if idx > 0 {
                        idx + m as i32
                    } else {
                        -1
                    }
                } else {
                    Self::sch(&nums[0..m], target)
                }
            }
        }
    }
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        Self::sch(&nums[..], target)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search() {
        assert_eq!(Solution::search(vec![3, 5, 1], 5), 1);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![1], 0), -1);
    }
}
