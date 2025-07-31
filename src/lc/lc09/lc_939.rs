// https://leetcode.com/problems/minimum-area-rectangle/
// 939. Minimum Area Rectangle
pub struct Solution;
impl Solution {
    pub fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
        let mut map = std::collections::HashMap::new();
        let mut points = points;
        points.sort();
        let mut min = i32::MAX;
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                if points[i][0] != points[j][0] {
                    break;
                }
                let y = (points[i][1], points[j][1]);
                if let Some(&v) = map.get(&y) {
                    min = min.min((points[i][0] - v) * (y.1 - y.0));
                }
                map.insert(y, points[i][0]);
            }
        }
        if min == i32::MAX {
            0
        } else {
            min
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_area_rect() {
        assert_eq!(
            Solution::min_area_rect(vec_vec![[1, 1], [1, 3], [3, 1], [3, 3], [2, 2]]),
            4
        );
        assert_eq!(
            Solution::min_area_rect(vec_vec![[1, 1], [1, 3], [3, 1], [3, 3], [4, 1], [4, 3]]),
            2
        );
    }
}
