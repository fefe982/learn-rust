// https://leetcode.com/problems/number-of-restricted-paths-from-first-to-last-node/
// 1786. Number of Restricted Paths From First to Last Nod
pub struct Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn count_restricted_paths(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let n = n as usize;
        let mut graph = vec![Vec::<(usize, i64)>::new(); n + 1];
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let w = e[2] as i64;
            graph[u].push((v, w));
            graph[v].push((u, w));
        }

        let inf = i64::MAX / 4;
        let mut dist = vec![inf; n + 1];
        dist[n] = 0;
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0i64), n));

        while let Some((Reverse(d), u)) = heap.pop() {
            if d != dist[u] {
                continue;
            }
            for &(v, w) in &graph[u] {
                let nd = d + w;
                if nd < dist[v] {
                    dist[v] = nd;
                    heap.push((Reverse(nd), v));
                }
            }
        }

        fn dfs(u: usize, n: usize, graph: &[Vec<(usize, i64)>], dist: &[i64], memo: &mut [i64]) -> i64 {
            if u == n {
                return 1;
            }
            if memo[u] != -1 {
                return memo[u];
            }
            let mut ways = 0i64;
            for &(v, _) in &graph[u] {
                if dist[u] > dist[v] {
                    ways += dfs(v, n, graph, dist, memo);
                    ways %= MOD;
                }
            }
            memo[u] = ways;
            ways
        }

        let mut memo = vec![-1i64; n + 1];
        dfs(1, n, &graph, &dist, &mut memo) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_restricted_paths() {
        assert_eq!(
            Solution::count_restricted_paths(
                5,
                vec_vec![
                    [1, 2, 3],
                    [1, 3, 3],
                    [2, 3, 1],
                    [1, 4, 2],
                    [5, 2, 2],
                    [3, 5, 1],
                    [5, 4, 10]
                ]
            ),
            3
        );
        assert_eq!(
            Solution::count_restricted_paths(
                7,
                vec_vec![
                    [1, 3, 1],
                    [4, 1, 2],
                    [7, 3, 4],
                    [2, 5, 3],
                    [5, 6, 1],
                    [6, 7, 2],
                    [7, 5, 3],
                    [2, 6, 4]
                ]
            ),
            1
        );
    }
}
