// https://leetcode.com/problems/find-weighted-median-node-in-tree/
// 3385. Find Weighted Median Node in Tree
pub struct Solution;
impl Solution {
    pub fn find_median(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push((e[1] as usize, e[2]));
            g[e[1] as usize].push((e[0] as usize, e[2]));
        }
        let bit = (usize::BITS - n.leading_zeros()) as usize;
        let mut parent = vec![vec![usize::MAX; bit]; n];
        let mut dis = vec![0; n];
        let mut depth = vec![0; n];
        fn dfs(
            g: &Vec<Vec<(usize, i32)>>,
            parent: &mut Vec<Vec<usize>>,
            dis: &mut Vec<i64>,
            depth: &mut Vec<i32>,
            cur: usize,
            fa: usize,
        ) {
            for &(to, w) in g[cur].iter() {
                if to == fa {
                    continue;
                }
                parent[to][0] = cur;
                dis[to] = dis[cur] + w as i64;
                depth[to] = depth[cur] + 1;
                dfs(g, parent, dis, depth, to, cur);
            }
        }
        dfs(&g, &mut parent, &mut dis, &mut depth, 0, usize::MAX);
        for i in 1..bit {
            for j in 0..n {
                let p = parent[j][i - 1];
                if p != usize::MAX {
                    parent[j][i] = parent[p][i - 1];
                }
            }
        }
        let kth_ancest = |mut x: usize, mut k: i32| -> usize {
            while k > 0 {
                let bit = k.trailing_zeros() as usize;
                x = parent[x][bit];
                k ^= 1 << bit;
            }
            x
        };
        let lca = |mut x: usize, mut y: usize| -> usize {
            let dx = depth[x];
            let dy = depth[y];
            if dx > dy {
                x = kth_ancest(x, dx - dy);
            } else if dx < dy {
                y = kth_ancest(y, dy - dx);
            }
            if x == y {
                return x;
            }
            let bit = (i32::BITS - dx.min(dy).leading_zeros()) as usize;
            for i in (0..bit).rev() {
                let px = parent[x][i];
                let py = parent[y][i];
                if px != py {
                    x = px;
                    y = py;
                }
            }
            parent[x][0]
        };
        let pdis = |mut x: usize, mut d: i64| -> usize {
            let bit = (i32::BITS - depth[x].leading_zeros()) as usize;
            for i in (0..bit).rev() {
                let p = parent[x][i];
                if p != usize::MAX && dis[x] - dis[p] <= d {
                    d -= dis[x] - dis[p];
                    x = p;
                }
            }
            x
        };
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            let x = q[0] as usize;
            let y = q[1] as usize;
            let xy = lca(x, y);
            let dx = dis[x] - dis[xy];
            let d = dx + dis[y] - dis[xy];
            let mut a;
            if 2 * dx > d {
                a = pdis(x, (d - 1) / 2);
                a = parent[a][0];
            } else {
                a = pdis(y, d / 2);
            }
            ans.push(a as i32);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_median() {
        assert_eq!(
            Solution::find_median(6, vec_vec![[0, 1, 7]], vec_vec![[1, 0], [0, 1]]),
            [0, 1]
        );
        assert_eq!(
            Solution::find_median(3, vec_vec![[0, 1, 2], [2, 0, 4]], vec_vec![[0, 1], [2, 0], [1, 2]]),
            [1, 0, 2]
        );
    }
}
