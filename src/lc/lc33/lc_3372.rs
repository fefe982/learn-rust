// https://leetcode.com/problems/maximize-the-number-of-target-nodes-after-connecting-trees-i/
// 3373. Maximize the Number of Target Nodes After Connecting Trees I
pub struct Solution;
impl Solution {
    fn walk(g: &Vec<Vec<usize>>, n: usize, p: usize, k: usize) -> i32 {
        if k == 0 {
            return 1;
        }
        let mut res = 1;
        for &c in g[n].iter() {
            if c != p {
                res += Self::walk(g, c, n, k - 1);
            }
        }
        res
    }
    fn convert(edges: Vec<Vec<i32>>, k: usize) -> Vec<i32> {
        let mut g = vec![vec![]; edges.len() + 1];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let mut cnt = Vec::with_capacity(g.len());
        for i in 0..g.len() {
            cnt.push(Self::walk(&g, i, usize::MAX, k));
        }
        cnt
    }
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut cnt1 = Self::convert(edges1, k);
        if k == 0 {
            return cnt1;
        }
        let max2 = Self::convert(edges2, k - 1).into_iter().max().unwrap_or(0);
        for i in 0..cnt1.len() {
            cnt1[i] += max2;
        }
        cnt1
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
                vec_vec![[0, 1], [0, 2], [0, 3], [2, 7], [1, 4], [4, 5], [4, 6]],
                2
            ),
            [9, 7, 9, 8, 8]
        );
        assert_eq!(
            Solution::max_target_nodes(
                vec_vec![[0, 1], [0, 2], [0, 3], [0, 4]],
                vec_vec![[0, 1], [1, 2], [2, 3]],
                1
            ),
            [6, 3, 3, 3, 3]
        );
    }
}
