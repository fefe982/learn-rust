// https://leetcode.com/problems/triangle/
// 120. Triangle
pub struct Solution;
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut triangle = triangle;
        for i in 1..triangle.len() {
            triangle[i][0] += triangle[i - 1][0];
            triangle[i][i] += triangle[i - 1][i - 1];
            for j in 1..i {
                triangle[i][j] += triangle[i - 1][j - 1].min(triangle[i - 1][j]);
            }
        }
        *triangle.last().unwrap().iter().min().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn minimum_total() {
        assert_eq!(
            Solution::minimum_total(vec_vec![[2], [3, 4], [6, 5, 7], [4, 1, 8, 3]]),
            11
        );
        assert_eq!(Solution::minimum_total(vec_vec![[-10]]), -10);
    }
}
