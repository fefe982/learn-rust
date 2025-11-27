// https://leetcode.com/problems/happy-students/
// 2860. Happy Students
pub struct Solution;
impl Solution {
    pub fn count_ways(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut ans = 0;
        if nums[0] > 0 {
            ans = 1;
        }
        if nums[nums.len() - 1] < nums.len() as i32 {
            ans += 1;
        }
        for i in 1..nums.len() {
            if i as i32 > nums[i - 1] && (i as i32) < nums[i] {
                ans += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_ways() {
        assert_eq!(Solution::count_ways(vec![1, 1]), 2);
        assert_eq!(Solution::count_ways(vec![6, 0, 3, 3, 6, 7, 2, 7]), 3);
    }
}
