// https://leetcode.com/problems/find-the-largest-area-of-square-inside-two-rectangles/
// 3047. Find the Largest Area of Square Inside Two Rectangles
pub struct Solution;
impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let mut mx = 0;
        let n = bottom_left.len();
        for i in 1..n {
            for j in 0..i {
                mx = mx.max(
                    (top_right[i][0].min(top_right[j][0]) - bottom_left[i][0].max(bottom_left[j][0]))
                        .min(top_right[i][1].min(top_right[j][1]) - bottom_left[i][1].max(bottom_left[j][1])),
                );
            }
        }
        mx as i64 * mx as i64
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn largest_square_area() {
        assert_eq!(
            Solution::largest_square_area(vec_vec![[3, 2], [1, 1]], vec_vec![[4, 5], [5, 4]]),
            1
        );
        assert_eq!(
            Solution::largest_square_area(vec_vec![[1, 1], [2, 2], [3, 1]], vec_vec![[3, 3], [4, 4], [6, 6]]),
            1
        );
        assert_eq!(
            Solution::largest_square_area(vec_vec![[1, 1], [1, 3], [1, 5]], vec_vec![[5, 5], [5, 7], [5, 9]]),
            4
        );
        assert_eq!(
            Solution::largest_square_area(vec_vec![[1, 1], [2, 2], [1, 2]], vec_vec![[3, 3], [4, 4], [3, 4]]),
            1
        );
        assert_eq!(
            Solution::largest_square_area(vec_vec![[1, 1], [3, 3], [3, 1]], vec_vec![[2, 2], [4, 4], [4, 2]]),
            0
        );
    }
}
