// https://leetcode.com/problems/minimum-time-difference/
// 539. Minimum Time Difference
pub struct Solution;
impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut time_points = time_points
            .into_iter()
            .map(|s| {
                let mut s = s.split(':');
                s.next().unwrap().parse::<i32>().unwrap() * 60 + s.next().unwrap().parse::<i32>().unwrap()
            })
            .collect::<Vec<_>>();
        time_points.sort_unstable();
        let mut delta = time_points[0] + 24 * 60 - time_points[time_points.len() - 1];
        for i in 1..time_points.len() {
            delta = delta.min(time_points[i] - time_points[i - 1]);
        }
        delta
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_min_difference() {
        assert_eq!(Solution::find_min_difference(vec_str!["23:59", "00:00"]), 1);
        assert_eq!(Solution::find_min_difference(vec_str!["00:00", "23:59", "00:00"]), 1);
    }
}
