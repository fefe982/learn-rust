// https://leetcode.com/problems/k-closest-points-to-origin/
// 973. K Closest Points to Origin
pub struct Solution;
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut points = points;
        points.sort_by_key(|p| p[0] * p[0] + p[1] * p[1]);
        points.truncate(k as usize);
        points
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn k_closest() {
        assert_eq!(Solution::k_closest(vec_vec![[1, 3], [-2, 2]], 1), vec_vec![[-2, 2]]);
        assert_sort_eq!(
            Solution::k_closest(vec_vec![[3, 3], [5, -1], [-2, 4]], 2),
            vec_vec![[3, 3], [-2, 4]]
        );
    }
}
