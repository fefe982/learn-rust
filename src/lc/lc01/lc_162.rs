// https://leetcode.com/problems/find-peak-element/
// 162. Find Peak Element
pub struct Solution;
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 0;
        }
        let mut left = 0;
        let mut right = n - 1;
        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if nums[mid] > nums[mid - 1] && nums[mid] > nums[mid + 1] {
                return mid as i32;
            }
            if nums[mid] > nums[mid + 1] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        if left == right {
            return left as i32;
        }
        if nums[left] > nums[right] {
            return left as i32;
        } else {
            return right as i32;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_peak_element() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
        assert_eq!(Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]), 5);
    }
}
