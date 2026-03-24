// https://leetcode.com/problems/find-nearest-point-that-has-the-same-x-or-y-coordinate/
// 1779. Find Nearest Point That Has the Same X or Y Coordinate
pub struct Solution;
impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut ans = -1;
        let mut dist = i32::MAX;
        for i in 0..points.len() {
            if points[i][0] == x || points[i][1] == y {
                let d = (x - points[i][0]).abs() + (y - points[i][1]).abs();
                if d < dist {
                    dist = d;
                    ans = i as i32;
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn nearest_valid_point() {
        assert_eq!(
            Solution::nearest_valid_point(3, 4, vec_vec![[1, 2], [3, 1], [2, 4], [2, 3], [4, 4]]),
            2
        );
        assert_eq!(Solution::nearest_valid_point(3, 4, vec_vec![[3, 4]]), 0);
        assert_eq!(Solution::nearest_valid_point(3, 4, vec_vec![[2, 3]]), -1);
    }
}
