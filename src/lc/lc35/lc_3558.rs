// https://leetcode.cn/problems/number-of-ways-to-assign-edge-weights-i/
// 3558. Number of Ways to Assign Edge Weights I
pub struct Solution;
impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![vec![]; edges.len() + 1];
        for e in edges {
            graph[e[0] as usize - 1].push(e[1] as usize - 1);
            graph[e[1] as usize - 1].push(e[0] as usize - 1);
        }
        let mut visited = vec![0; graph.len()];
        visited[0] = 1;
        let mut q = std::collections::VecDeque::new();
        q.push_back(0);
        let mut max = 1;
        while let Some(u) = q.pop_front() {
            for &v in &graph[u] {
                if visited[v] == 0 {
                    visited[v] = visited[u] + 1;
                    max = max.max(visited[v]);
                    q.push_back(v);
                }
            }
        }
        max -= 2;
        let mut ans = 1;
        let mut p2 = 2;
        let modulo = 1_000_000_007i64;
        while max > 0 {
            if max % 2 == 1 {
                ans = (ans * p2) % modulo;
            }
            p2 = (p2 * p2) % modulo;
            max /= 2;
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_assign_edge_weights() {
        assert_eq!(Solution::assign_edge_weights(vec_vec![[1, 2]]), 1);
        assert_eq!(
            Solution::assign_edge_weights(vec_vec![[1, 2], [1, 3], [3, 4], [3, 5]]),
            2
        );
    }
}
