// https://leetcode.com/problems/find-the-kth-largest-integer-in-the-array/
// 1985. Find the Kth Largest Integer in the Array
pub struct Solution;
impl Solution {
    pub fn kth_largest_number(nums: Vec<String>, k: i32) -> String {
        let mut nums = nums;
        nums.sort_by(|a, b| {
            if a.len() != b.len() {
                a.len().cmp(&b.len())
            } else {
                a.cmp(b)
            }
        });
        nums[nums.len() - k as usize].clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn kth_largest_number() {
        assert_eq!(
            Solution::kth_largest_number(vec_str!["3", "6", "7", "10"], 4),
            "3".to_string()
        );
        assert_eq!(
            Solution::kth_largest_number(vec_str!["2", "21", "12", "1"], 3),
            "2".to_string()
        );
        assert_eq!(Solution::kth_largest_number(vec_str!["0", "0"], 2), "0".to_string());
    }
}
