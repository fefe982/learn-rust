// https://leetcode.com/problems/shortest-path-in-a-weighted-tree/
// 3515. Shortest Path in a Weighted Tree
pub struct Solution;
impl Solution {
    fn add(val: &mut Vec<i32>, idx: usize, il: usize, ir: usize, l: usize, r: usize, v: i32) {
        if r <= il || ir <= l {
            return;
        }
        if l <= il && ir <= r {
            val[idx] += v;
            return;
        }
        let mid = (il + ir) / 2;
        val[2 * idx + 1] += val[idx];
        val[2 * idx + 2] += val[idx];
        val[idx] = 0;
        Self::add(val, 2 * idx + 1, il, mid, l, r, v);
        Self::add(val, 2 * idx + 2, mid, ir, l, r, v);
    }
    fn walk(
        g: &Vec<Vec<(usize, i32)>>,
        weight: &mut Vec<i32>,
        rng: &mut Vec<[usize; 2]>,
        idx: usize,
        cur: usize,
        p: usize,
        w: i32,
        val: &mut Vec<i32>,
    ) -> usize {
        let s = idx;
        let mut idx = idx + 1;
        for &(n, wn) in g[cur].iter() {
            if n == p {
                continue;
            }
            weight[n] = wn;
            idx = Self::walk(g, weight, rng, idx, n, cur, w + wn, val);
        }
        rng[cur] = [s, idx];
        let nv = val.len();
        val[nv / 2 - 1 + s] = w;
        idx
    }
    fn query(val: &mut Vec<i32>, idx: usize, il: usize, ir: usize, i: usize) -> i32 {
        if il + 1 == ir {
            return val[idx];
        }
        if val[idx] != 0 {
            val[2 * idx + 1] += val[idx];
            val[2 * idx + 2] += val[idx];
            val[idx] = 0;
        }
        let mid = (il + ir) / 2;
        if i < mid {
            Self::query(val, 2 * idx + 1, il, mid, i)
        } else {
            Self::query(val, 2 * idx + 2, mid, ir, i)
        }
    }
    pub fn tree_queries(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for edge in edges {
            let (a, b, w) = (edge[0] as usize - 1, edge[1] as usize - 1, edge[2]);
            g[a].push((b, w));
            g[b].push((a, w));
        }
        let mut rng = vec![[0, 0]; n];
        let mut weight = vec![0; n];
        let sz = (2 << (usize::BITS - (n - 1).leading_zeros())) as usize;
        let mut val = vec![0; sz];
        Self::walk(&g, &mut weight, &mut rng, 0, 0, n, 0, &mut val);
        let mut ans = Vec::with_capacity(queries.len());
        for query in queries {
            if query[0] == 1 {
                let u = query[1] as usize - 1;
                let v = query[2] as usize - 1;
                let w = if rng[u][0] > rng[v][0] { u } else { v };
                let diff = query[3] - weight[w];
                if diff != 0 {
                    Self::add(&mut val, 0, 0, sz / 2, rng[w][0], rng[w][1], diff);
                }
                weight[w] = query[3];
            } else {
                let u = query[1] as usize - 1;
                ans.push(Self::query(&mut val, 0, 0, sz / 2, rng[u][0]));
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn tree_queries() {
        assert_eq!(
            Solution::tree_queries(
                4,
                vec_vec![[1, 2, 2], [2, 3, 3], [3, 4, 4]],
                vec_vec![[2, 4], [1, 2, 3, 10], [2, 3], [1, 2, 3, 1], [2, 3]]
            ),
            [9, 12, 3]
        );
        assert_eq!(
            Solution::tree_queries(2, vec_vec![[1, 2, 7]], vec_vec![[2, 2], [1, 1, 2, 4], [2, 2]]),
            [7, 4]
        );
        assert_eq!(
            Solution::tree_queries(
                3,
                vec_vec![[1, 2, 2], [1, 3, 4]],
                vec_vec![[2, 1], [2, 3], [1, 1, 3, 7], [2, 2], [2, 3]]
            ),
            [0, 4, 2, 7]
        );
        assert_eq!(
            Solution::tree_queries(
                4,
                vec_vec![[1, 2, 2], [2, 3, 1], [3, 4, 5]],
                vec_vec![[2, 4], [2, 3], [1, 2, 3, 3], [2, 2], [2, 3]]
            ),
            [8, 3, 2, 5]
        );
    }
}
