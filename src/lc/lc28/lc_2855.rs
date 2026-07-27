// https://leetcode.com/problems/minimum-right-shifts-to-sort-the-array/
// 2855. Minimum Right Shifts to Sort the Array
pub struct Solution;
impl Solution {
    pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
        let mut valley = 0;
        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                if valley != 0 {
                    return -1;
                }
                valley = i;
            }
        }
        if valley == 0 {
            return 0;
        }
        if nums[0] < nums[nums.len() - 1] {
            return -1;
        }
        nums.len() as i32 - valley as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_right_shifts() {
        assert_eq!(Solution::minimum_right_shifts(vec![3, 4, 5, 1, 2]), 2);
        assert_eq!(Solution::minimum_right_shifts(vec![1, 3, 5]), 0);
        assert_eq!(Solution::minimum_right_shifts(vec![2, 1, 4]), -1);
    }
}
