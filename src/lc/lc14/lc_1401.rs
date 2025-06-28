// https://leetcode.com/problems/circle-and-rectangle-overlapping/
// 1401. Circle and Rectangle Overlapping
pub struct Solution;
impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        let dx = if x2 < x_center {
            x_center - x2
        } else if x1 > x_center {
            x1 - x_center
        } else {
            0
        };
        let dy = if y2 < y_center {
            y_center - y2
        } else if y1 > y_center {
            y1 - y_center
        } else {
            0
        };
        dx * dx + dy * dy <= radius * radius
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_overlap() {
        assert_eq!(Solution::check_overlap(1, 0, 0, 1, -1, 3, 1), true);
        assert_eq!(Solution::check_overlap(1, 1, 1, 1, -3, 2, -1), false);
        assert_eq!(Solution::check_overlap(1, 0, 0, -1, 0, 0, 1), true);
    }
}
