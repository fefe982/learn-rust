// https://leetcode.com/problems/minimum-weighted-subgraph-with-the-required-paths-ii/
// 3553. Minimum Weighted Subgraph with the Required Paths II
pub struct Solution;
impl Solution {
    fn dfs(
        g: &Vec<Vec<(usize, i32)>>,
        p: &mut Vec<Vec<usize>>,
        depth: &mut Vec<i32>,
        dis: &mut Vec<i32>,
        l: i32,
        d: i32,
        u: usize,
        fa: usize,
    ) {
        p[u][0] = fa;
        dis[u] = d;
        depth[u] = l;
        for i in 1..p[u].len() {
            p[u][i] = p[p[u][i - 1]][i - 1];
        }
        for &(v, duv) in &g[u] {
            if v != fa {
                Self::dfs(g, p, depth, dis, l + 1, d + duv, v, u);
            }
        }
    }
    fn distance(p: &Vec<Vec<usize>>, depth: &Vec<i32>, dis: &Vec<i32>, mut u: usize, mut v: usize) -> i32 {
        let length = dis[u] + dis[v];
        let mut du = depth[u];
        let mut dv = depth[v];
        if du < dv {
            std::mem::swap(&mut u, &mut v);
            std::mem::swap(&mut du, &mut dv);
        }
        let mut diff = du - dv;
        while diff > 0 {
            let i = diff.trailing_zeros() as usize;
            u = p[u][i];
            diff = (diff - 1) & diff;
        }
        if u != v {
            for i in (0..p[0].len()).rev() {
                if p[u][i] != p[v][i] {
                    u = p[u][i];
                    v = p[v][i];
                }
            }
            u = p[u][0];
        }
        length - 2 * dis[u]
    }
    pub fn minimum_weight(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut g = vec![vec![]; n + 1];
        for edge in edges {
            g[edge[0] as usize + 1].push((edge[1] as usize + 1, edge[2]));
            g[edge[1] as usize + 1].push((edge[0] as usize + 1, edge[2]));
        }
        let mut depth = vec![0; n + 1];
        let mut dis = vec![0; n + 1];
        let s = (usize::BITS - n.leading_zeros()) as usize;
        let mut p = vec![vec![0; s]; n + 1];
        Self::dfs(&g, &mut p, &mut depth, &mut dis, 0, 0, 1, 0);
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            ans.push(
                (Self::distance(&p, &depth, &dis, q[0] as usize + 1, q[1] as usize + 1)
                    + Self::distance(&p, &depth, &dis, q[1] as usize + 1, q[2] as usize + 1)
                    + Self::distance(&p, &depth, &dis, q[0] as usize + 1, q[2] as usize + 1))
                    / 2,
            );
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn minimum_weight() {
        assert_eq!(
            Solution::minimum_weight(
                vec_vec![[0, 1, 2], [1, 2, 3], [1, 3, 5], [1, 4, 4], [2, 5, 6]],
                vec_vec![[2, 3, 4], [0, 2, 5]]
            ),
            [12, 11]
        );
        assert_eq!(
            Solution::minimum_weight(vec_vec![[1, 0, 8], [0, 2, 7]], vec_vec![[0, 1, 2]]),
            [15]
        );
    }
}
