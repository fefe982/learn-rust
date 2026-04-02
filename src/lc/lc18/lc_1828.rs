// https://leetcode.com/problems/queries-on-number-of-points-inside-a-circle/
// 1828. Queries on Number of Points Inside a Circle
pub struct Solution;
impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        queries.into_iter().map(|q| {
            let (qx, qy, r2) = (q[0], q[1], q[2] * q[2]);
            points.iter().filter(|p| {
                let dx = p[0] - qx;
                let dy = p[1] - qy;
                dx * dx + dy * dy <= r2
            }).count() as i32
        }).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_points() {
        assert_eq!(
            Solution::count_points(
                vec_vec![[1, 3], [3, 3], [5, 3], [2, 2]],
                vec_vec![[2, 3, 1], [4, 3, 1], [1, 1, 2]]
            ),
            vec![3, 2, 2]
        );
        assert_eq!(
            Solution::count_points(
                vec_vec![[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]],
                vec_vec![[1, 2, 2], [2, 2, 2], [4, 3, 2], [4, 3, 3]]
            ),
            vec![2, 3, 2, 4]
        );
    }
}
