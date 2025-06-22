// https://leetcode.com/problems/rectangle-area/
// 223. Rectangle Area
pub struct Solution;
impl Solution {
    pub fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
        let cx1 = ax1.max(bx1);
        let cy1 = ay1.max(by1);
        let cx2 = ax2.min(bx2);
        let cy2 = ay2.min(by2);
        (ax2 - ax1) * (ay2 - ay1) + (bx2 - bx1) * (by2 - by1) - ((cx2 - cx1).max(0) * (cy2 - cy1).max(0))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_area() {
        assert_eq!(Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
        assert_eq!(Solution::compute_area(-2, -2, 2, 2, -2, -2, 2, 2), 16);
    }
}
