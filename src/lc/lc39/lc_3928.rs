// https://leetcode.com/problems/minimum-cost-to-buy-apples-ii/
// 3928. Minimum Cost to Buy Apples II
pub struct Solution;
impl Solution {
    pub fn min_cost(n: i32, prices: Vec<i32>, roads: Vec<Vec<i32>>) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let n = n as usize;
        let mut g_empty = vec![vec![]; n];
        let mut g_carry = vec![vec![]; n];

        for r in roads {
            let u = r[0] as usize;
            let v = r[1] as usize;
            let c = r[2] as i64;
            let t = r[3] as i64;

            g_empty[u].push((v, c));
            g_empty[v].push((u, c));

            let w = c * t;
            g_carry[u].push((v, w));
            g_carry[v].push((u, w));
        }

        fn dijkstra(start: usize, g: &[Vec<(usize, i64)>]) -> Vec<i64> {
            let n = g.len();
            let inf = i64::MAX / 4;
            let mut dist = vec![inf; n];
            let mut pq = BinaryHeap::new();

            dist[start] = 0;
            pq.push((Reverse(0_i64), start));

            while let Some((Reverse(d), u)) = pq.pop() {
                if d != dist[u] {
                    continue;
                }
                for &(v, w) in &g[u] {
                    let nd = d + w;
                    if nd < dist[v] {
                        dist[v] = nd;
                        pq.push((Reverse(nd), v));
                    }
                }
            }

            dist
        }

        let all_empty: Vec<Vec<i64>> = (0..n).map(|s| dijkstra(s, &g_empty)).collect();
        let all_carry: Vec<Vec<i64>> = (0..n).map(|s| dijkstra(s, &g_carry)).collect();

        let inf = i64::MAX / 4;
        let mut ans = vec![0; n];
        for i in 0..n {
            let mut best = prices[i] as i64;
            for j in 0..n {
                if all_empty[i][j] == inf || all_carry[i][j] == inf {
                    continue;
                }
                let candidate = prices[j] as i64 + all_empty[i][j] + all_carry[i][j];
                if candidate < best {
                    best = candidate;
                }
            }
            ans[i] = best as i32;
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_cost() {
        assert_eq!(Solution::min_cost(2, vec![8, 3], vec_vec![[0, 1, 1, 2]]), vec![6, 3]);
        assert_eq!(
            Solution::min_cost(3, vec![9, 4, 6], vec_vec![[0, 1, 1, 3], [1, 2, 4, 2]]),
            vec![8, 4, 6]
        );
        assert_eq!(
            Solution::min_cost(3, vec![10, 11, 1], vec_vec![[0, 2, 1, 3], [1, 2, 3, 4], [0, 1, 5, 2]]),
            vec![5, 11, 1]
        );
    }
}
