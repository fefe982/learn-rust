// https://leetcode.com/problems/minimize-the-maximum-difference-of-pairs/
// 2616. Minimize the Maximum Difference of Pairs
pub struct Solution;
impl Solution {
    fn check(nums: &Vec<i32>, bound: i32, p: i32) -> bool {
        let mut skip = false;
        let mut cnt = 0;
        for i in 1..nums.len() {
            if skip {
                skip = false;
                continue;
            }
            if nums[i] - nums[i - 1] <= bound {
                cnt += 1;
                skip = true;
            }
        }
        cnt >= p
    }
    pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
        if p == 0 {
            return 0;
        }
        nums.sort_unstable();
        let mut low = i32::MAX;
        let mut high = 0;
        for i in 1..nums.len() {
            let diff = nums[i] - nums[i - 1];
            high = high.max(diff);
            low = low.min(diff);
        }
        if Self::check(&nums, low, p) {
            return low;
        }
        while low + 1 < high {
            let mid = (low + high) / 2;
            if Self::check(&nums, mid, p) {
                high = mid;
            } else {
                low = mid;
            }
        }
        high
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimize_max() {
        assert_eq!(Solution::minimize_max(vec![10, 1, 2, 7, 1, 3], 2), 1);
        assert_eq!(Solution::minimize_max(vec![4, 2, 1, 2], 1), 0);
    }
}
