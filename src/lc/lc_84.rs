// https://leetcode.com/problems/largest-rectangle-in-histogram/
// 84. Largest Rectangle in Histogram
pub struct Solution;
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack = vec![(0, 0)];
        let mut max_area = 0;
        for (&h, idx) in heights.iter().chain([0].iter()).zip(1..) {
            let mut start = idx;
            while stack.last().unwrap().0 > h {
                let last = stack.pop().unwrap();
                max_area = std::cmp::max(max_area, last.0 * (idx - last.1));
                start = last.1
            }
            if stack.last().unwrap().0 < h {
                stack.push((h, start));
            }
        }
        max_area
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn largest_rectangle_area() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    }
}
