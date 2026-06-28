// https://leetcode.com/problems/minimum-time-to-reach-target-with-limited-power/
// 3977. Minimum Time to Reach Target with Limited Power
pub struct Solution;
impl Solution {
    pub fn min_time_max_power(
        n: i32,
        edges: Vec<Vec<i32>>,
        power: i32,
        cost: Vec<i32>,
        source: i32,
        target: i32,
    ) -> Vec<i64> {
        if source == target {
            return vec![0, power as i64];
        }
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for edge in edges {
            graph[edge[0] as usize].push((edge[1] as usize, edge[2] as i64));
        }
        let power = power as usize;
        let mut dist = vec![vec![i64::MAX; power + 1]; n];
        let source = source as usize;
        let target = target as usize;
        dist[source][power] = 0;
        let mut heap = std::collections::BinaryHeap::new();
        heap.push((std::cmp::Reverse(0), power, source));
        while let Some((std::cmp::Reverse(d), p, u)) = heap.pop() {
            if u == target {
                return vec![d, p as i64];
            }
            if d > dist[u][p] || p < cost[u] as usize {
                continue;
            }
            let np = p - cost[u] as usize;
            for &(v, w) in &graph[u] {
                if d + w < dist[v][np] {
                    dist[v][np] = d + w;
                    heap.push((std::cmp::Reverse(dist[v][np]), np, v));
                }
            }
        }
        vec![-1, -1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_time_max_power() {
        assert_eq!(
            Solution::min_time_max_power(
                5,
                vec_vec![[0, 1, 1], [1, 4, 1], [0, 2, 1], [2, 3, 1], [3, 4, 1]],
                4,
                vec![2, 3, 1, 1, 1],
                0,
                4
            ),
            vec![3, 0]
        );
        assert_eq!(
            Solution::min_time_max_power(3, vec_vec![[0, 1, 2], [1, 2, 2], [2, 0, 2]], 3, vec![1, 1, 1], 1, 1),
            vec![0, 3]
        );
        assert_eq!(
            Solution::min_time_max_power(4, vec_vec![[0, 1, 3], [2, 3, 4]], 3, vec![1, 1, 1, 1], 0, 3),
            vec![-1, -1]
        );
    }
}
