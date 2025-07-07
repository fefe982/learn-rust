// https://leetcode.com/problems/find-center-of-star-graph/
// 1791. Find Center of Star Graph
pub struct Solution;
impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        if edges[0][0] == edges[1][0] || edges[0][0] == edges[1][1] {
            edges[0][0]
        } else {
            edges[0][1]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_center() {
        assert_eq!(Solution::find_center(vec_vec![[1, 2], [2, 3], [4, 2]]), 2);
        assert_eq!(Solution::find_center(vec_vec![[1, 2], [5, 1], [1, 3], [1, 4]]), 1);
    }
}
