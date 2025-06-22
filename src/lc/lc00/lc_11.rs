// https://leetcode.com/problems/container-with-most-water/
// 11. Container With Most Water
pub struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut end = height.len() - 1;
        let mut max = 0;
        while start < end {
            max = max.max((end - start) as i32 * height[start].min(height[end]));
            if height[start] < height[end] {
                start += 1;
            } else {
                end -= 1;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_area() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
}
