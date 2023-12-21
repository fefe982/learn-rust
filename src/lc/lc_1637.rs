// https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/
// 1637. Widest Vertical Area Between Two Points Containing No Points
pub struct Solution;
impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort();
        let mut max_dist = 0;
        for idx in 0..points.len() - 1 {
            max_dist = std::cmp::max(points[idx + 1][0] - points[idx][0], max_dist);
        }
        max_dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_width_of_vertical_area() {
        assert_eq!(
            Solution::max_width_of_vertical_area(vec![vec![8, 7], vec![9, 9], vec![7, 4], vec![9, 7]]),
            1
        );
        assert_eq!(
            Solution::max_width_of_vertical_area(vec![
                vec![3, 1],
                vec![9, 0],
                vec![1, 0],
                vec![1, 4],
                vec![5, 3],
                vec![8, 8]
            ]),
            3
        );
    }
}
