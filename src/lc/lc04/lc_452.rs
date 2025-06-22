// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/
// 452. Minimum Number of Arrows to Burst Balloons
pub struct Solution;
impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_unstable();
        let mut count = 1;
        let mut prev = points[0][1];
        for i in 1..points.len() {
            if points[i][0] > prev {
                count += 1;
                prev = points[i][1];
            } else {
                prev = prev.min(points[i][1]);
            }
        }
        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_min_arrow_shots() {
        assert_eq!(
            Solution::find_min_arrow_shots(vec_vec![[10, 16], [2, 8], [1, 6], [7, 12]]),
            2
        );
        assert_eq!(
            Solution::find_min_arrow_shots(vec_vec![[1, 2], [3, 4], [5, 6], [7, 8]]),
            4
        );
        assert_eq!(
            Solution::find_min_arrow_shots(vec_vec![[1, 2], [2, 3], [3, 4], [4, 5]]),
            2
        );
    }
}
