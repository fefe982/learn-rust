// https://leetcode.com/problems/maximum-number-of-visible-points/
// 1610. Maximum Number of Visible Points
pub struct Solution;
impl Solution {
    pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
        let mut origin = 0;
        let angle = (angle as f64) * std::f64::consts::PI / 180_f64;
        let mut vec = Vec::new();
        for point in points {
            if point == location {
                origin += 1;
            } else {
                vec.push(((point[0] - location[0]) as f64).atan2((point[1] - location[1]) as f64));
            }
        }
        vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        let epsilon = 1e-5;
        let mut y = 0usize;
        let mut max = 0usize;
        let len = vec.len();
        for (x, &a) in vec.iter().enumerate() {
            while y < x + len
                && ((vec[y % len] - a).rem_euclid(std::f64::consts::PI * 2.0) - angle) < epsilon
            {
                y += 1;
            }
            max = std::cmp::max(y - x, max);
            if max == len {
                break;
            }
        }
        max as i32 + origin
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn visible_points() {
        assert_eq!(
            Solution::visible_points(vec![vec![2, 1], vec![2, 2], vec![3, 3]], 90, vec![1, 1]),
            3
        );
        assert_eq!(
            Solution::visible_points(
                vec![vec![2, 1], vec![2, 2], vec![3, 4], vec![1, 1]],
                90,
                vec![1, 1]
            ),
            4
        );
        assert_eq!(
            Solution::visible_points(vec![vec![1, 0], vec![2, 1]], 13, vec![1, 1]),
            1
        );
    }
}
