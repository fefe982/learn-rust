// https://leetcode.com/problems/largest-triangle-area/
// 812. Largest Triangle Area
pub struct Solution;
impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut points = points;
        points.sort();
        let mut hull = Vec::with_capacity(points.len() + 1);
        hull.push((points[0][0], points[0][1]));
        let cross = |p0: (i32, i32), p1: (i32, i32), p2: &Vec<i32>| {
            (p1.0 - p0.0) * (p2[1] - p1.1) - (p1.1 - p0.1) * (p2[0] - p1.0)
        };
        for i in 1..points.len() {
            while hull.len() > 1 && cross(hull[hull.len() - 2], hull[hull.len() - 1], &points[i]) <= 0 {
                hull.pop();
            }
            hull.push((points[i][0], points[i][1]));
        }
        let m = hull.len();
        for i in (0..points.len() - 1).rev() {
            while hull.len() > m && cross(hull[hull.len() - 2], hull[hull.len() - 1], &points[i]) <= 0 {
                hull.pop();
            }
            hull.push((points[i][0], points[i][1]));
        }
        hull.pop();
        let mut max_area: f64 = 0.0;
        for i in 0..hull.len() - 2 {
            for j in i + 1..hull.len() - 1 {
                let mut max_local = 0;
                for k in j + 1..hull.len() {
                    let area = ((hull[i].0 - hull[j].0) * (hull[k].1 - hull[i].1)
                        - (hull[i].1 - hull[j].1) * (hull[k].0 - hull[i].0))
                        .abs();
                    if area < max_local {
                        break;
                    }
                    max_local = area;
                }
                max_area = max_area.max(max_local as f64 / 2.0);
            }
        }
        max_area
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn largest_triangle_area() {
        assert_approx_eq!(
            Solution::largest_triangle_area(vec_vec![[0, 0], [0, 1], [1, 0], [0, 2], [2, 0]]),
            2.0,
            1e-5
        );
        assert_approx_eq!(
            Solution::largest_triangle_area(vec_vec![[1, 0], [0, 0], [0, 1]]),
            0.5,
            1e-5
        );
    }
}
