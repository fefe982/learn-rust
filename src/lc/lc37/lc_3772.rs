// https://leetcode.com/problems/maximum-subgraph-score-in-a-tree/
// 3772. Maximum Subgraph Score in a Tree
pub struct Solution;
impl Solution {
    fn dfs(g: &Vec<Vec<usize>>, good: &Vec<i32>, u: usize, p: usize, msub: &mut Vec<i32>) {
        let mut sum = 0;
        for &v in &g[u] {
            if v != p {
                Self::dfs(g, good, v, u, msub);
                sum += msub[v];
            }
        }
        msub[u] = (sum + good[u]).max(0);
    }
    fn sum(g: &Vec<Vec<usize>>, good: &Vec<i32>, u: usize, p: usize, msub: &mut Vec<i32>, ans: &mut Vec<i32>) {
        let mut sum = 0;
        for &v in &g[u] {
            sum += msub[v];
        }
        ans[u] = sum + good[u];
        let save = msub[u];
        for &v in &g[u] {
            if v != p {
                msub[u] = (sum - msub[v] + good[u]).max(0);
                Self::sum(g, good, v, u, msub, ans);
            }
        }
        msub[u] = save;
    }
    pub fn max_subgraph_score(n: i32, edges: Vec<Vec<i32>>, good: Vec<i32>) -> Vec<i32> {
        let mut good = good;
        let n = n as usize;
        for i in 0..n {
            good[i] = if good[i] == 1 { 1 } else { -1 };
        }
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let mut msub = vec![0; n];
        Self::dfs(&g, &good, 0, n, &mut msub);
        let mut ans = vec![0; n];
        Self::sum(&g, &good, 0, n, &mut msub, &mut ans);
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_subgraph_score() {
        assert_eq!(
            Solution::max_subgraph_score(3, vec_vec![[0, 1], [1, 2]], vec![1, 0, 1]),
            [1, 1, 1]
        );
        assert_eq!(
            Solution::max_subgraph_score(5, vec_vec![[1, 0], [1, 2], [1, 3], [3, 4]], vec![0, 1, 0, 1, 1]),
            [2, 3, 2, 3, 3]
        );
        assert_eq!(Solution::max_subgraph_score(2, vec_vec![[0, 1]], vec![0, 0]), [-1, -1]);
    }
}
