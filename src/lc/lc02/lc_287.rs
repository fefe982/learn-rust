// https://leetcode.com/problems/find-the-duplicate-number/
// 287. Find the Duplicate Number
pub struct Solution;
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = 0;
        let mut fast = 0;
        loop {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];
            if slow == fast {
                break;
            }
        }
        fast = 0;
        while fast != slow {
            fast = nums[fast as usize];
            slow = nums[slow as usize];
        }
        fast
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_duplicate() {
        assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    }
}
