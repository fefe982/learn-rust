// https://leetcode.com/problems/number-of-ways-to-assign-edge-weights-ii/
// 3559. Number of Ways to Assign Edge Weights II
pub struct Solution;
impl Solution {
    fn dfs(g: &Vec<Vec<usize>>, p: &mut Vec<Vec<usize>>, d: &mut Vec<i32>, l: i32, u: usize, fa: usize) {
        p[u][0] = fa;
        d[u] = l;
        for i in 1..p[u].len() {
            p[u][i] = p[p[u][i - 1]][i - 1];
        }
        for &v in &g[u] {
            if v != fa {
                Self::dfs(g, p, d, l + 1, v, u);
            }
        }
    }
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut g = vec![vec![]; n + 1];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let s = (usize::BITS - n.leading_zeros()) as usize;
        let mut p = vec![vec![0; s]; n + 1];
        let mut d = vec![0; n + 1];
        Self::dfs(&g, &mut p, &mut d, 0, 1, 0);
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            let mut u = q[0] as usize;
            let mut v = q[1] as usize;
            if u == v {
                ans.push(0);
                continue;
            }
            let mut duv = 0;
            let mut du = d[u];
            let mut dv = d[v];
            if du < dv {
                std::mem::swap(&mut u, &mut v);
                std::mem::swap(&mut du, &mut dv);
            }
            if du > dv {
                let mut l = du - dv;
                while l > 0 {
                    u = p[u][l.trailing_zeros() as usize];
                    l = (l - 1) & l;
                }
                duv = (du - dv) as i32;
            }
            if u != v {
                for i in (0..s).rev() {
                    if p[u][i] != p[v][i] {
                        u = p[u][i];
                        v = p[v][i];
                        duv += 2 << i;
                    }
                }
                duv += 2;
            }
            duv -= 1;
            let mut r = 1;
            let mut pow = 2;
            const MOD: i64 = 1000000007;
            while duv > 0 {
                if duv & 1 == 1 {
                    r = (r * pow) % MOD;
                }
                duv >>= 1;
                pow = (pow * pow) % MOD;
            }
            ans.push(r as i32);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn assign_edge_weights() {
        assert_eq!(
            Solution::assign_edge_weights(vec_vec![[1, 2]], vec_vec![[1, 1], [1, 2]]),
            [0, 1]
        );
        assert_eq!(
            Solution::assign_edge_weights(
                vec_vec![[1, 2], [1, 3], [3, 4], [3, 5]],
                vec_vec![[1, 4], [3, 4], [2, 5]]
            ),
            [2, 1, 4]
        );
    }
}
