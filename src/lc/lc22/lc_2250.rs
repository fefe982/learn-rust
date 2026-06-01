// https://leetcode.com/problems/count-number-of-rectangles-containing-each-point/
// 2250. Count Number of Rectangles Containing Each Point
pub struct Solution;
impl Solution {
    pub fn count_rectangles(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32> {
        let mut rectangles = rectangles;
        rectangles.sort_unstable_by_key(|r| -r[1]);
        let mut points = points.into_iter().enumerate().collect::<Vec<_>>();
        points.sort_unstable_by_key(|p| -p.1[1]);
        let mut ans = vec![0; points.len()];
        let mut i = 0;
        let mut heights = vec![];
        for (idx, point) in points {
            let (x, y) = (point[0], point[1]);
            while i < rectangles.len() && rectangles[i][1] >= y {
                heights.push(rectangles[i][0]);
                i += 1;
            }
            heights.sort_unstable();
            ans[idx] = (heights.len() - heights.partition_point(|&j| j < x)) as i32;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_rectangles() {
        assert_eq!(
            Solution::count_rectangles(vec_vec![[1, 2], [2, 3], [2, 5]], vec_vec![[2, 1], [1, 4]]),
            vec![2, 1]
        );
        assert_eq!(
            Solution::count_rectangles(vec_vec![[1, 1], [2, 2], [3, 3]], vec_vec![[1, 3], [1, 1]]),
            vec![1, 3]
        );
    }
}
