// https://leetcode.com/problems/find-polygon-with-the-largest-perimeter/
// 2971. Find Polygon With the Largest Perimeter
pub struct Solution;
impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut max = -1;
        let mut sum = nums[0] as i64 + nums[1] as i64;
        for n in nums.into_iter().skip(2) {
            if sum > n as i64 {
                max = sum + n as i64;
            }
            sum += n as i64;
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_largest_perimeter() {
        assert_eq!(Solution::largest_perimeter(vec![5, 5, 5]), 15);
        assert_eq!(Solution::largest_perimeter(vec![1, 12, 1, 2, 5, 50, 3]), 12);
        assert_eq!(Solution::largest_perimeter(vec![5, 5, 50]), -1);
    }
}
