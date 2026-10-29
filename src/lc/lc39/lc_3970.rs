// https://leetcode.cn/problems/shortest-path-with-at-most-k-consecutive-identical-characters/
// 3970. Shortest Path With At Most K Consecutive Identical Characters
pub struct Solution;
impl Solution {
    pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push((e[1] as usize, e[2]));
        }
        let labels = labels.as_bytes();
        let mut h = std::collections::BinaryHeap::new();
        h.push(std::cmp::Reverse((0, 0, 0)));
        let k = k as usize;
        let mut dist = vec![vec![i32::MAX; k]; n];
        dist[0][0] = 0;
        while let Some(std::cmp::Reverse((d, u, c))) = h.pop() {
            if u == n - 1 {
                return d;
            }
            if d > dist[u][c] {
                continue;
            }
            for &(v, w) in &g[u] {
                let cc = if labels[v] == labels[u] { c + 1 } else { 0 };
                if cc >= k {
                    continue;
                }
                let nd = d + w;
                if nd >= dist[v][cc] {
                    continue;
                }
                dist[v][cc] = nd;
                h.push(std::cmp::Reverse((nd, v, cc)));
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn shortest_path() {
        assert_eq!(
            Solution::shortest_path(3, vec_vec![[0, 1, 1], [1, 2, 1], [0, 2, 3]], "aab".to_string(), 1),
            3
        );
        assert_eq!(
            Solution::shortest_path(3, vec_vec![[0, 1, 1], [1, 2, 1], [0, 2, 3]], "aab".to_string(), 2),
            2
        );
        assert_eq!(
            Solution::shortest_path(3, vec_vec![[0, 1, 1], [1, 2, 1]], "aaa".to_string(), 2),
            -1
        );
    }
}
