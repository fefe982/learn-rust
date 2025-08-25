// https://leetcode.com/problems/maximum-area-of-longest-diagonal-rectangle/
// 3000. Maximum Area of Longest Diagonal Rectangle
pub struct Solution;
impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut max_area = 0;
        let mut max_diag = 0;
        for d in dimensions {
            let diag = d[0] * d[0] + d[1] * d[1];
            if diag > max_diag {
                max_diag = diag;
                max_area = d[0] * d[1];
            } else if diag == max_diag {
                max_area = max_area.max(d[0] * d[1]);
            }
        }
        max_area
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn area_of_max_diagonal() {
        assert_eq!(Solution::area_of_max_diagonal(vec_vec![[9, 3], [8, 6]]), 48);
        assert_eq!(Solution::area_of_max_diagonal(vec_vec![[3, 4], [4, 3]]), 12);
    }
}
