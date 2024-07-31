// https://leetcode.com/problems/minimum-rectangles-to-cover-points/
// 3111. Minimum Rectangles to Cover Points
pub struct Solution;
impl Solution {
    pub fn min_rectangles_to_cover_points(points: Vec<Vec<i32>>, w: i32) -> i32 {
        let mut p = points.into_iter().map(|v| v[0]).collect::<Vec<_>>();
        p.sort_unstable();
        p.into_iter()
            .fold((-w - 1, 0), |(l, c), x| if l + w < x { (x, c + 1) } else { (l, c) })
            .1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_rectangles_to_cover_points() {
        assert_eq!(
            Solution::min_rectangles_to_cover_points(vec_vec![[2, 1], [1, 0], [1, 4], [1, 8], [3, 5], [4, 6]], 1),
            2
        );
        assert_eq!(
            Solution::min_rectangles_to_cover_points(
                vec_vec![[0, 0], [1, 1], [2, 2], [3, 3], [4, 4], [5, 5], [6, 6]],
                2
            ),
            3
        );
        assert_eq!(Solution::min_rectangles_to_cover_points(vec_vec![[2, 3], [1, 2]], 0), 2);
    }
}
