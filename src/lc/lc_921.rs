// https://leetcode.com/problems/sort-an-array/
// 912. Sort an Array
pub struct Solution;
impl Solution {
    fn sort(nums: &mut [i32]) {
        if nums.len() <= 1 {
            return;
        }
        let p = nums[0];
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            while l < r && nums[r] >= p {
                r -= 1;
            }
            if l < r {
                nums[l] = nums[r];
            }
            while l < r && nums[l] <= p {
                l += 1;
            }
            if l < r {
                nums[r] = nums[l];
            }
        }
        nums[l] = p;
        while l > 0 && nums[l] == p {
            l -= 1;
        }
        while r < nums.len() - 1 && nums[r] == p {
            r += 1;
        }
        Self::sort(&mut nums[0..=l]);
        Self::sort(&mut nums[r..]);
    }
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        Self::sort(&mut nums[..]);
        nums
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sort_array() {
        assert_eq!(Solution::sort_array(vec![5, 2, 3, 1]), [1, 2, 3, 5]);
        assert_eq!(Solution::sort_array(vec![5, 1, 1, 2, 0, 0]), [0, 0, 1, 1, 2, 5]);
    }
}
