// https://leetcode.com/problems/best-position-for-a-service-centre/
// 1515. Best Position for a Service Centre
pub struct Solution;
impl Solution {
    fn dist(positions: &Vec<Vec<i32>>, pos: (f64, f64)) -> f64 {
        positions.iter().fold(0.0, |acc, p| {
            acc + ((p[0] as f64 - pos.0).powi(2) + (p[1] as f64 - pos.1).powi(2)).sqrt()
        })
    }
    pub fn get_min_dist_sum(positions: Vec<Vec<i32>>) -> f64 {
        let mut pos = (50.0, 50.0);
        let mut step = 25.0;
        let mut min_dist = Self::dist(&positions, pos);
        'w: while step > 1e-6 {
            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let new_pos = (pos.0 + step * dx as f64, pos.1 + step * dy as f64);
                let new_dist = Self::dist(&positions, new_pos);
                if new_dist < min_dist {
                    pos = new_pos;
                    min_dist = new_dist;
                    continue 'w;
                }
            }
            step /= 2.;
        }
        min_dist
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn test_get_min_dist_sum() {
        assert_approx_eq!(
            Solution::get_min_dist_sum(vec_vec![[0, 1], [1, 0], [1, 2], [2, 1]]),
            4.0,
            1e-5
        );
        assert_approx_eq!(Solution::get_min_dist_sum(vec_vec![[1, 1], [3, 3]]), 2.82843, 1e-5);
    }
}
