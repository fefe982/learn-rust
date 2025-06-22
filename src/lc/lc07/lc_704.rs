// https://leetcode.com/problems/binary-search/
// 704. Binary Search
pub struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut i = 0;
        let mut j = nums.len() - 1;
        while i < j {
            let mid = (i + j) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[mid] < target {
                i = mid + 1;
            }
            if nums[mid] > target {
                if mid == 0 {
                    return -1;
                }
                j = mid - 1;
            }
        }
        if i == j && nums[i] == target {
            i as i32
        } else {
            -1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
