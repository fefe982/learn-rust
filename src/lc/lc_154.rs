// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/solutions/
// 154. Find Minimum in Rotated Sorted Array II
pub struct Solution;
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut high = nums.len() - 1;
        if nums[low] < nums[high] {
            return nums[low];
        }
        while high - low > 1 {
            let mid = (low + high) / 2;
            if nums[mid] > nums[low] {
                low = mid;
            } else if nums[mid] < nums[high] {
                high = mid;
            } else if nums[low] == nums[high] {
                high -= 1;
            } else if nums[mid] == nums[low] {
                low = mid;
            } else if nums[mid] == nums[high] {
                high = mid;
            }
        }
        std::cmp::min(nums[low], nums[high])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_min() {
        assert_eq!(Solution::find_min(vec![1, 3, 5]), 1);
        assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
    }
}
