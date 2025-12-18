// https://leetcode.cn/problems/you-le-yuan-de-you-lan-ji-hua/
// LCP 16. 游乐园的游览计划
pub struct Solution;
impl Solution {
    pub fn max_weight(edges: Vec<Vec<i32>>, value: Vec<i32>) -> i32 {
        let nv = value.len();
        let ne = edges.len();
        let mut cnt = vec![0; nv];
        let mut edgeid = vec![vec![]; nv];
        let mut edgemap = std::collections::HashMap::<(usize, usize), usize>::new();
        for e in &edges {
            cnt[e[0] as usize] += 1;
            cnt[e[1] as usize] += 1;
        }
        let mut g = vec![vec![]; nv];
        for (i, e) in edges.iter().enumerate() {
            let mut e0 = e[0] as usize;
            let mut e1 = e[1] as usize;
            edgeid[e0].push(i);
            edgeid[e1].push(i);
            edgemap.insert((e0, e1), i);
            edgemap.insert((e1, e0), i);
            if cnt[e1] > cnt[e0] || (cnt[e1] == cnt[e0] && e1 > e0) {
                std::mem::swap(&mut e0, &mut e1);
            }
            g[e0].push((e1, i));
        }
        type Triangle = (i32, usize, usize, usize);
        let mut top_triagle_per_side = vec![vec![(0, 0, 0, 0); 4]; ne];
        let mut trianle_per_node = vec![vec![]; nv];
        let mut vis = vec![(usize::MAX, usize::MAX); nv];
        fn add_side(top_triagle_per_side: &mut Vec<Vec<Triangle>>, ei: usize, v: i32, e0: usize, e1: usize, e2: usize) {
            let mut ins = 3;
            for i in (0..3).rev() {
                if top_triagle_per_side[ei][i].0 < v {
                    top_triagle_per_side[ei][i + 1] = top_triagle_per_side[ei][i];
                    ins = i;
                } else {
                    break;
                }
            }
            if ins < 3 {
                top_triagle_per_side[ei][ins] = (v, e0, e1, e2);
            }
        }
        for (i, e) in edges.iter().enumerate() {
            let e0 = e[0] as usize;
            let e1 = e[1] as usize;
            for &(e2, ei) in &g[e0] {
                vis[e2] = (i, ei);
            }
            for &(e2, ei) in &g[e1] {
                if vis[e2].0 == i {
                    let v = value[e0] + value[e1] + value[e2];
                    trianle_per_node[e0].push((v, e0, e1, e2));
                    trianle_per_node[e1].push((v, e0, e1, e2));
                    trianle_per_node[e2].push((v, e0, e1, e2));
                    add_side(&mut top_triagle_per_side, i, v, e0, e1, e2);
                    add_side(&mut top_triagle_per_side, vis[e2].1, v, e0, e1, e2);
                    add_side(&mut top_triagle_per_side, ei, v, e0, e1, e2);
                }
            }
        }
        let mut ans = 0;
        for i in 0..ne {
            if top_triagle_per_side[i][0].0 == 0 {
                continue;
            }
            ans = ans.max(top_triagle_per_side[i][0].0);
            if top_triagle_per_side[i][1].0 == 0 {
                continue;
            }
            ans = ans.max(
                top_triagle_per_side[i][0].0 + top_triagle_per_side[i][1].0
                    - value[edges[i][0] as usize]
                    - value[edges[i][1] as usize],
            );
        }
        let combine = |ta: &Triangle, tb: &Triangle| -> i32 {
            if ta.0 == 0 || tb.0 == 0 {
                return 0;
            }
            let mut v = ta.0 + tb.0;
            if tb.1 == ta.1 || tb.1 == ta.2 || tb.1 == ta.3 {
                v -= value[tb.1];
            }
            if tb.2 == ta.1 || tb.2 == ta.2 || tb.2 == ta.3 {
                v -= value[tb.2];
            }
            if tb.3 == ta.1 || tb.3 == ta.2 || tb.3 == ta.3 {
                v -= value[tb.3];
            }
            v
        };
        for i in 0..nv {
            if edgeid[i].len() == 0 {
                continue;
            }
            let mut maxei = edgeid[i][0];
            for j in 1..edgeid[i].len() {
                if top_triagle_per_side[maxei][0].0 < top_triagle_per_side[edgeid[i][j]][0].0 {
                    maxei = edgeid[i][j];
                }
            }
            if top_triagle_per_side[maxei][0].0 == 0 {
                continue;
            }
            let mut p = top_triagle_per_side[maxei][0].1;
            let mut q = top_triagle_per_side[maxei][0].2;
            let mut r = top_triagle_per_side[maxei][0].3;
            if q == i {
                std::mem::swap(&mut p, &mut q);
            }
            if r == i {
                std::mem::swap(&mut p, &mut r);
            }
            for t in &trianle_per_node[i] {
                ans = ans.max(combine(t, &top_triagle_per_side[maxei][0]));
            }
            let ej = edgemap[&(q, i)];
            let ek = edgemap[&(r, i)];
            ans = ans.max(combine(&top_triagle_per_side[ej][1], &top_triagle_per_side[ek][1]));
            ans = ans.max(combine(&top_triagle_per_side[ej][1], &top_triagle_per_side[ek][2]));
            ans = ans.max(combine(&top_triagle_per_side[ej][2], &top_triagle_per_side[ek][1]));
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test() {
        assert_eq!(Solution::max_weight(vec_vec![[0, 1], [1, 2], [0, 2]], vec![1, 2, 3]), 6);
        assert_eq!(Solution::max_weight(vec_vec![[0, 2], [2, 1]], vec![1, 2, 5]), 0);
        assert_eq!(
            Solution::max_weight(
                vec_vec![
                    [0, 1],
                    [0, 2],
                    [0, 3],
                    [0, 4],
                    [0, 5],
                    [1, 3],
                    [2, 4],
                    [2, 5],
                    [3, 4],
                    [3, 5],
                    [4, 5]
                ],
                vec![7, 8, 6, 8, 9, 7]
            ),
            39
        );
    }
}
