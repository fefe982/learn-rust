// https://leetcode.com/problems/reachable-nodes-in-subdivided-graph/
// 882. Reachable Nodes In Subdivided Graph
pub struct Solution;
impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        let mut graph = vec![vec![]; n as usize];
        for edge in edges {
            graph[edge[0] as usize].push((edge[1], edge[2]));
            graph[edge[1] as usize].push((edge[0], edge[2]));
        }
        let mut sum = 0;
        let mut visited = vec![false; n as usize];
        let mut used = std::collections::HashMap::<(i32, i32), i32>::new();
        let mut q = std::collections::BinaryHeap::new();
        q.push(std::cmp::Reverse((0, 0, -1, 0)));
        while let Some(std::cmp::Reverse((dist, cost, from, node))) = q.pop() {
            if used.contains_key(&(from, node)) {
                continue;
            }
            let mut uncoverable = 0;
            if let Some(&u) = used.get(&(node, from)) {
                uncoverable = u;
            }
            let c = cost - uncoverable;
            let base = dist - cost - 1;
            let max = (base + c).min(max_moves);
            sum += max - base;
            used.insert((from, node), max - base);
            if visited[node as usize] {
                continue;
            }
            visited[node as usize] = true;
            if dist > max_moves {
                continue;
            }
            sum += 1;
            for &(next, cost) in graph[node as usize].iter() {
                q.push(std::cmp::Reverse((dist + cost + 1, cost, node, next)));
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_reachable_nodes() {
        assert_eq!(
            Solution::reachable_nodes(vec_vec![[1, 2, 5], [0, 3, 3], [1, 3, 2], [2, 3, 4], [0, 4, 1]], 7, 5),
            13
        );
        assert_eq!(
            Solution::reachable_nodes(vec_vec![[0, 1, 10], [0, 2, 1], [1, 2, 2]], 6, 3),
            13
        );
        assert_eq!(
            Solution::reachable_nodes(vec_vec![[0, 1, 4], [1, 2, 6], [0, 2, 8], [1, 3, 1]], 10, 4),
            23
        );
        assert_eq!(
            Solution::reachable_nodes(vec_vec![[1, 2, 4], [1, 4, 5], [1, 3, 1], [2, 3, 4], [3, 4, 5]], 17, 5),
            1
        );
    }
}
