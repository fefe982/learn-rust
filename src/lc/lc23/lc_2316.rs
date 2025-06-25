// https://leetcode.com/problems/count-unreachable-pairs-of-nodes-in-an-undirected-graph/
// 2316. Count Unreachable Pairs of Nodes in an Undirected Graph
pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        if n <= 1 {
            return 0;
        }
        if edges.len() > (n - 1) * (n - 2) / 2 {
            return 0;
        }
        let mut edge_map: HashMap<i32, Vec<i32>> = HashMap::new();
        for edge in &edges {
            edge_map.entry(edge[0]).or_default().push(edge[1]);
            edge_map.entry(edge[1]).or_default().push(edge[0]);
        }
        let mut blocks = Vec::new();
        let mut visited = vec![false; n];
        for idx in 0..(n as i32) {
            if visited[idx as usize] {
                continue;
            }
            let mut cnt = 0;
            let mut que = vec![idx];
            while let Some(node) = que.pop() {
                if visited[node as usize] {
                    continue;
                }
                visited[node as usize] = true;
                cnt += 1;
                if let Some(node_vec) = edge_map.get(&node) {
                    for &nn in node_vec {
                        que.push(nn);
                    }
                }
            }
            blocks.push(cnt)
        }
        if blocks.len() == 1 {
            return 0;
        }
        let mut sum = 0;
        for i in 0..blocks.len() {
            sum += blocks[i] as i64 * (n - blocks[i]) as i64;
        }
        sum / 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_pairs() {
        assert_eq!(
            Solution::count_pairs(3, vec![vec![0, 1], vec![0, 2], vec![2, 3]]),
            0
        );
        assert_eq!(
            Solution::count_pairs(
                7,
                vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]]
            ),
            14
        );
    }
}
