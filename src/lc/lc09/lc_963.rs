// https://leetcode.com/problems/minimum-area-rectangle-ii/
// 963. Minimum Area Rectangle II
pub struct Solution;
impl Solution {
    pub fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
        let mut ans = f64::MAX;
        let mut map = std::collections::HashMap::new();
        let len = |x, y| x as i64 * x as i64 + y as i64 * y as i64;
        for i in 0..points.len() - 1 {
            for j in i + 1..points.len() {
                let key = (
                    (points[i][0] + points[j][0], points[i][1] + points[j][1]),
                    len(points[i][0] - points[j][0], points[i][1] - points[j][1]),
                );
                map.entry(key).or_insert(vec![]).push(i);
            }
        }
        for ((_, l), v) in map {
            if v.len() > 1 {
                for i in 0..v.len() - 1 {
                    let p1 = v[i];
                    for j in i + 1..v.len() {
                        let p2 = v[j];
                        let sides = len(points[p1][0] - points[p2][0], points[p1][1] - points[p2][1]);
                        ans = ans.min((sides as f64) * ((l - sides) as f64));
                    }
                }
            }
        }
        if ans == f64::MAX {
            0.0
        } else {
            ans.sqrt()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use assert_approx_eq::*;
    #[test]
    fn min_area_free_rect() {
        assert_approx_eq!(
            Solution::min_area_free_rect(vec_vec![[1, 2], [2, 1], [1, 0], [0, 1]]),
            2.0,
            1e-5
        );
        assert_approx_eq!(
            Solution::min_area_free_rect(vec_vec![[0, 1], [2, 1], [1, 1], [1, 0], [2, 0]]),
            1.0,
            1e-5
        );
        assert_approx_eq!(
            Solution::min_area_free_rect(vec_vec![[0, 3], [1, 2], [3, 1], [1, 3], [2, 1]]),
            0.0,
            1e-5
        );
    }
}
