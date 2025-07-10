// https://leetcode.com/problems/minimum-time-to-visit-disappearing-nodes/
// 3112. Minimum Time to Visit Disappearing Nodes
pub struct Solution;
impl Solution {
    pub fn minimum_time(n: i32, edges: Vec<Vec<i32>>, disappear: Vec<i32>) -> Vec<i32> {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for edge in edges {
            g[edge[0] as usize].push((edge[1] as usize, edge[2]));
            g[edge[1] as usize].push((edge[0] as usize, edge[2]));
        }
        let mut ans = vec![-1; n];
        let mut q = std::collections::BinaryHeap::new();
        q.push((std::cmp::Reverse(0), 0));
        while let Some((std::cmp::Reverse(d), u)) = q.pop() {
            if ans[u] >= 0 {
                continue;
            }
            ans[u] = d;
            for &(n, e) in &g[u] {
                if ans[n] == -1 && d + e < disappear[n] {
                    q.push((std::cmp::Reverse(d + e), n));
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_minimum_time() {
        assert_eq!(
            Solution::minimum_time(3, vec_vec![[0, 1, 2], [1, 2, 1], [0, 2, 4]], vec![1, 1, 5]),
            [0, -1, 4]
        );
        assert_eq!(Solution::minimum_time(2, vec_vec![[0, 1, 1]], vec![1, 1]), [0, -1]);
    }
}
