// https://leetcode.com/problems/type-of-triangle/
// 3024. Type of Triangle
pub struct Solution;
impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        if nums[0] == nums[1] && nums[1] == nums[2] {
            return "equilateral".to_string();
        }
        let mut nums = nums;
        nums.sort();
        if nums[0] + nums[1] <= nums[2] {
            return "none".to_string();
        }
        if nums[0] == nums[1] || nums[1] == nums[2] {
            return "isosceles".to_string();
        }
        "scalene".to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn triangle_type() {
        assert_eq!(Solution::triangle_type(vec![3, 3, 3]), "equilateral".to_string());
        assert_eq!(Solution::triangle_type(vec![3, 4, 5]), "scalene".to_string());
    }
}
