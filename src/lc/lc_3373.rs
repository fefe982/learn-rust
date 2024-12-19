// https://leetcode.com/problems/maximize-the-number-of-target-nodes-after-connecting-trees-ii/
// 3373. Maximize the Number of Target Nodes After Connecting Trees II
pub struct Solution;
impl Solution {
    fn walk(g: &Vec<Vec<usize>>, n: usize, p: usize, even: usize, res: &mut Vec<Vec<usize>>) {
        res[even].push(n);
        for &c in g[n].iter() {
            if c != p {
                Self::walk(g, c, n, even ^ 1, res);
            }
        }
    }
    fn convert(edges: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        let mut g = vec![vec![]; edges.len() + 1];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let mut node = vec![vec![]; 2];
        Self::walk(&g, 0, 0, 0, &mut node);
        node
    }
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![0; edges1.len() + 1];
        let n1 = Self::convert(edges1);
        let n2 = Self::convert(edges2);
        let add = n2[0].len().max(n2[1].len());
        for i in 0..2 {
            for &n in &n1[i] {
                res[n] = (n1[i].len() + add) as i32;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_target_nodes() {
        assert_eq!(
            Solution::max_target_nodes(
                vec_vec![[0, 1], [0, 2], [2, 3], [2, 4]],
                vec_vec![[0, 1], [0, 2], [0, 3], [2, 7], [1, 4], [4, 5], [4, 6]]
            ),
            [8, 7, 7, 8, 8]
        );
        assert_eq!(
            Solution::max_target_nodes(
                vec_vec![[0, 1], [0, 2], [0, 3], [0, 4]],
                vec_vec![[0, 1], [1, 2], [2, 3]],
            ),
            [3, 6, 6, 6, 6]
        );
    }
}
