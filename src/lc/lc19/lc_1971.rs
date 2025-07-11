// https://leetcode.com/problems/find-if-path-exists-in-graph/
// 1971. Find if Path Exists in Graph
pub struct Solution;
impl Solution {
    fn get_p(v: &mut Vec<usize>, x: usize) -> usize {
        let mut px = v[x];
        while px != v[px] {
            px = v[px];
        }
        v[x] = px;
        px
    }
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut p = (0..n as usize).collect::<Vec<_>>();
        for v in edges {
            let (a, b) = (Self::get_p(&mut p, v[0] as usize), Self::get_p(&mut p, v[1] as usize));
            p[a] = b;
        }
        Self::get_p(&mut p, source as usize) == Self::get_p(&mut p, destination as usize)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_valid_path() {
        assert_eq!(Solution::valid_path(3, vec_vec![[0, 1], [1, 2], [2, 0]], 0, 2), true);
        assert_eq!(
            Solution::valid_path(6, vec_vec![[0, 1], [0, 2], [3, 5], [5, 4], [4, 3]], 0, 5),
            false
        );
    }
}
