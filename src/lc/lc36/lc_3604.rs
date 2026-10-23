// https://leetcode.com/problems/minimum-time-to-reach-destination-in-directed-graph/
// 3604. Minimum Time to Reach Destination in Directed Graph
pub struct Solution;
impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for edge in edges {
            graph[edge[0] as usize].push((edge[1] as usize, edge[2], edge[3]));
        }
        let mut visited = vec![false; n];
        let mut queue = std::collections::BinaryHeap::new();
        queue.push(std::cmp::Reverse((0, 0)));
        while let Some(std::cmp::Reverse((time, node))) = queue.pop() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            if node == n - 1 {
                return time;
            }
            for &(next, start, end) in graph[node].iter() {
                if time > end {
                    continue;
                }
                queue.push(std::cmp::Reverse((time.max(start) + 1, next)));
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
    fn min_time() {
        assert_eq!(Solution::min_time(3, vec_vec![[0, 1, 0, 1], [1, 2, 2, 5]]), 3);
        assert_eq!(
            Solution::min_time(4, vec_vec![[0, 1, 0, 3], [1, 3, 7, 8], [0, 2, 1, 5], [2, 3, 4, 7]]),
            5
        );
        assert_eq!(Solution::min_time(3, vec_vec![[1, 0, 1, 3], [1, 2, 3, 5]]), -1);
    }
}
