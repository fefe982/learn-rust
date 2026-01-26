// https://leetcode.com/problems/minimum-cost-path-with-edge-reversals/
// 3650. Minimum Cost Path with Edge Reversals
pub struct Solution;
impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in edges {
            let (u, v, w) = (e[0] as usize, e[1] as usize, e[2]);
            g[u].push((v, w));
            g[v].push((u, w * 2));
        }
        let mut q = std::collections::BinaryHeap::new();
        q.push((std::cmp::Reverse(0), 0));
        let mut visited = vec![false; n];
        while let Some((std::cmp::Reverse(cost), u)) = q.pop() {
            if visited[u] {
                continue;
            }
            if u == n - 1 {
                return cost;
            }
            visited[u] = true;
            for &(v, w) in &g[u] {
                if !visited[v] {
                    q.push((std::cmp::Reverse(cost + w), v));
                }
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
    fn min_cost() {
        assert_eq!(Solution::min_cost(3, vec_vec![[2, 1, 1], [1, 0, 1], [2, 0, 16]]), 4);
        assert_eq!(
            Solution::min_cost(4, vec_vec![[0, 1, 3], [3, 1, 1], [2, 3, 4], [0, 2, 2]]),
            5
        );
        assert_eq!(
            Solution::min_cost(4, vec_vec![[0, 2, 1], [2, 1, 1], [1, 3, 1], [2, 3, 3]]),
            3
        );
    }
}
