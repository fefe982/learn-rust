// https://leetcode.com/problems/count-the-number-of-good-nodes/
// 3249. Count the Number of Good Nodes in Binary Tree
pub struct Solution;
impl Solution {
    fn count(g: &Vec<Vec<usize>>, p: usize, n: usize) -> (i32, i32) {
        let mut nchild = 0;
        let mut nnodes = 0;
        let mut minnodes = g.len() as i32 + 1;
        let mut gchild = 0;
        for &i in &g[n] {
            if i == p {
                continue;
            }
            let (cg, cn) = Self::count(g, n, i);
            nchild += 1;
            nnodes += cn;
            minnodes = minnodes.min(cn);
            gchild += cg;
        }
        if minnodes * nchild == nnodes {
            (gchild + 1, nnodes + 1)
        } else {
            (gchild, nnodes + 1)
        }
    }
    pub fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32 {
        let mut g = vec![vec![]; edges.len() + 1];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        Self::count(&g, usize::MAX, 0).0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_good_nodes() {
        assert_eq!(
            Solution::count_good_nodes(vec_vec![[2, 0], [4, 2], [1, 2], [3, 1], [5, 1]]),
            5
        );
        assert_eq!(
            Solution::count_good_nodes(vec_vec![[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]]),
            7
        );
        assert_eq!(
            Solution::count_good_nodes(vec_vec![[0, 1], [1, 2], [2, 3], [3, 4], [0, 5], [1, 6], [2, 7], [3, 8]]),
            6
        );
    }
}
