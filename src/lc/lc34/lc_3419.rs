// https://leetcode.com/problems/minimize-the-maximum-edge-weight-of-graph/
// 3419. Minimize the Maximum Edge Weight of a Tree
pub struct Solution;
impl Solution {
    fn dfs(graph: &Vec<Vec<(usize, i32)>>, threshold: i32) -> bool {
        let mut visited = vec![false; graph.len()];
        let mut cnt = 1;
        visited[0] = true;
        let mut q = vec![0];
        while let Some(node) = q.pop() {
            for &(next, weight) in &graph[node] {
                if !visited[next] && weight <= threshold {
                    visited[next] = true;
                    cnt += 1;
                    q.push(next);
                }
            }
        }
        cnt == graph.len()
    }
    pub fn min_max_weight(n: i32, edges: Vec<Vec<i32>>, _threshold: i32) -> i32 {
        let mut high = 0;
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for edge in edges {
            let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
            graph[v].push((u, w));
            high = high.max(w);
        }
        if !Self::dfs(&graph, high) {
            return -1;
        }
        let mut low = 0;
        while low + 1 < high {
            let mid = (low + high) / 2;
            if Self::dfs(&graph, mid) {
                high = mid;
            } else {
                low = mid;
            }
        }
        high
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_max_weight() {
        assert_eq!(
            Solution::min_max_weight(5, vec_vec![[1, 0, 1], [2, 0, 2], [3, 0, 1], [4, 3, 1], [2, 1, 1]], 2),
            1
        );
        assert_eq!(
            Solution::min_max_weight(
                5,
                vec_vec![[0, 1, 1], [0, 2, 2], [0, 3, 1], [0, 4, 1], [1, 2, 1], [1, 4, 1]],
                1
            ),
            -1
        );
        assert_eq!(
            Solution::min_max_weight(
                5,
                vec_vec![[1, 2, 1], [1, 3, 3], [1, 4, 5], [2, 3, 2], [3, 4, 2], [4, 0, 1]],
                1
            ),
            2
        );
        assert_eq!(
            Solution::min_max_weight(5, vec_vec![[1, 2, 1], [1, 3, 3], [1, 4, 5], [2, 3, 2], [4, 0, 1]], 1),
            -1
        );
    }
}
