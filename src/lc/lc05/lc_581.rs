// https://leetcode.com/problems/shortest-unsorted-continuous-subarray/
// 581. Shortest Unsorted Continuous Subarray
pub struct Solution;
impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut i = 1;
        while i < nums.len() {
            if nums[i] >= nums[i - 1] {
                i += 1;
            } else {
                break;
            }
        }
        if i == nums.len() {
            return 0;
        }
        let mut llen = i;
        let mut max = nums[i - 1];
        let mut rlen = 0;
        while i < nums.len() {
            while llen > 0 {
                if nums[i] < nums[llen - 1] {
                    llen -= 1;
                } else {
                    break;
                }
            }
            if nums[i] >= max {
                rlen += 1;
                max = nums[i];
            } else {
                rlen = 0;
            }
            i += 1;
        }
        (nums.len() - llen - rlen) as i32
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn find_unsorted_subarray() {
        assert_eq!(super::Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]), 5);
        assert_eq!(super::Solution::find_unsorted_subarray(vec![1, 2, 3, 4]), 0);
        assert_eq!(super::Solution::find_unsorted_subarray(vec![1]), 0);
    }
}
