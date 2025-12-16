// https://leetcode.cn/problems/Za25hA/
// LCP 21. 追逐游戏
pub struct Solution;
impl Solution {
    pub fn chase_game(edges: Vec<Vec<i32>>, start_a: i32, start_b: i32) -> i32 {
        let n = edges.len();
        let mut g = vec![vec![]; n];
        let sa = start_a as usize - 1;
        let sb = start_b as usize - 1;
        for edge in edges {
            let s = edge[0] as usize - 1;
            let t = edge[1] as usize - 1;
            g[s].push(t);
            g[t].push(s);
            if (s == sa && t == sb) || (s == sb && t == sa) {
                return 1;
            }
        }
        let mut nloop = 0;
        let mut depth = vec![(usize::MAX, usize::MAX); n];
        let mut in_loop = vec![false; n];
        let mut q = vec![];
        q.push((usize::MAX, 0, 1));
        'd: while let Some((p, u, d)) = q.pop() {
            if depth[u].0 == usize::MAX {
                depth[u] = (d, p);
            }
            for &n in &g[u] {
                if depth[n].0 == usize::MAX {
                    q.push((u, n, d + 1));
                } else if n == p {
                    continue;
                } else {
                    in_loop[u] = true;
                    in_loop[n] = true;
                    in_loop[p] = true;
                    let mut pp = depth[p].1;
                    while !in_loop[pp] {
                        in_loop[pp] = true;
                        pp = depth[pp].1;
                    }
                    nloop = depth[u].0 - depth[n].0 + 1;
                    break 'd;
                }
            }
        }
        let mut da = vec![usize::MAX; n];
        let mut db = vec![usize::MAX; n];
        let dd = |s: usize, ds: &mut Vec<usize>| -> usize {
            let mut q = std::collections::VecDeque::new();
            q.push_back((usize::MAX, s, 0));
            let mut l = usize::MAX;
            while let Some((p, u, d)) = q.pop_front() {
                if ds[u] != usize::MAX {
                    continue;
                }
                if l == usize::MAX && in_loop[u] {
                    l = u;
                }
                ds[u] = d;
                for &n in &g[u] {
                    if n == p {
                        continue;
                    }
                    q.push_back((u, n, d + 1));
                }
            }
            l
        };
        dd(sa, &mut da);
        let lb = dd(sb, &mut db);
        if nloop > 3 && db[lb] + 1 < da[lb] {
            return -1;
        }
        let mut ans = 0;
        for i in 0..n {
            if da[i] > db[i] + 1 {
                ans = ans.max(da[i]);
            }
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::chase_game(
                vec_vec![
                    [1, 2],
                    [2, 3],
                    [3, 4],
                    [2, 5],
                    [3, 6],
                    [5, 7],
                    [7, 8],
                    [5, 9],
                    [5, 10],
                    [1, 10]
                ],
                3,
                8
            ),
            4
        );
        assert_eq!(
            Solution::chase_game(vec_vec![[1, 2], [2, 3], [3, 4], [4, 1], [2, 5], [5, 6]], 3, 5),
            3
        );
        assert_eq!(Solution::chase_game(vec_vec![[1, 2], [2, 3], [3, 4], [4, 1]], 1, 3), -1);
        assert_eq!(Solution::chase_game(vec_vec![[1, 2], [2, 3], [3, 4], [4, 1]], 1, 3), -1);
    }
}
