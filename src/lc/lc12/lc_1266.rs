// https://leetcode.com/problems/minimum-time-visiting-all-points/
// 1266. Minimum Time Visiting All Points
pub struct Solution;
impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        points
            .into_iter()
            .fold((0, None), |(acc, last): (i32, Option<Vec<i32>>), p| {
                if let Some(l) = last {
                    (acc + (p[0] - l[0]).abs().max((p[1] - l[1]).abs()), Some(p))
                } else {
                    (0, Some(p))
                }
            })
            .0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_time_to_visit_all_points() {
        assert_eq!(
            Solution::min_time_to_visit_all_points(vec_vec![[1, 1], [3, 4], [-1, 0]]),
            7,
        );
        assert_eq!(Solution::min_time_to_visit_all_points(vec_vec![[3, 2], [-2, 2]]), 5,);
    }
}
