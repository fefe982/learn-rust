// https://leetcode.com/problems/split-array-into-consecutive-subsequences/
// 659. Split Array into Consecutive Subsequences
pub struct Solution;
impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut s = std::collections::VecDeque::new();
        let mut i = 0;
        let mut last = i32::MIN;
        loop {
            if i == nums.len() || nums[i] != last + 1 {
                while let Some(&v) = s.back() {
                    if last - v < 2 {
                        return false;
                    }
                    s.pop_back();
                }
            }
            if i == nums.len() {
                return true;
            }
            let mut j = i + 1;
            while j < nums.len() && nums[j] == nums[i] {
                j += 1;
            }
            while s.len() > j - i {
                let v = s.pop_front().unwrap();
                if last - v < 2 {
                    return false;
                }
            }
            while s.len() < j - i {
                s.push_back(nums[i]);
            }
            last = nums[i];
            i = j;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_possible() {
        assert_eq!(Solution::is_possible(vec![1, 2, 3, 3, 4, 5]), true);
        assert_eq!(Solution::is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5]), true);
        assert_eq!(Solution::is_possible(vec![1, 2, 3, 4, 4, 5]), false);
    }
}
