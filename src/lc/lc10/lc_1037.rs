// https://leetcode.com/problems/valid-boomerang/
// 1037. Valid Boomerang
pub struct Solution;
impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        if points[0] == points[1] || points[0] == points[2] || points[1] == points[2] {
            return false;
        }
        let a = (points[1][1] - points[0][1]) * (points[2][0] - points[0][0]);
        let b = (points[2][1] - points[0][1]) * (points[1][0] - points[0][0]);
        a != b
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn is_boomerang() {
        assert_eq!(Solution::is_boomerang(vec_vec![[1, 1], [2, 3], [3, 2]]), true);
        assert_eq!(Solution::is_boomerang(vec_vec![[1, 1], [2, 2], [3, 3]]), false);
    }
}
