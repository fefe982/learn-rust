// https://leetcode.com/problems/minimum-edge-weight-equilibrium-queries-in-a-tree/
// 2846. Minimum Edge Weight Equilibrium Queries in a Tree
pub struct Solution;
impl Solution {
    pub fn min_operations_queries(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut pre = vec![0; n as usize];
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push((e[1] as usize, e[2]));
            g[e[1] as usize].push((e[0] as usize, e[2]));
        }
        let mut visited = vec![0; n];
        let mut q = vec![0];
        let mut cnt = vec![vec![]; n];
        cnt[0] = vec![0; 26];
        while let Some(v) = q.pop() {
            visited[v] = -1;
            for &(u, w) in g[v].iter() {
                if visited[u] == 0 {
                    pre[u] = v;
                    cnt[u] = cnt[v].clone();
                    cnt[u][w as usize - 1] += 1;
                    q.push(u);
                }
            }
        }
        g.clear();
        let mut ans = vec![0; queries.len()];
        for (i, q) in queries.into_iter().enumerate() {
            let q0 = q[0] as usize;
            let q1 = q[1] as usize;
            let i = i as i32;
            let mut p = q0;
            visited[q0] = i;
            while p != 0 {
                p = pre[p];
                visited[p] = i;
            }
            p = q1;
            while p != 0 {
                if visited[p] == i {
                    break;
                }
                p = pre[p];
            }
            let mut sum = 0;
            let mut max = 0;
            for i in 0..26 {
                let c = cnt[q0][i] + cnt[q1][i] - 2 * cnt[p][i];
                max = max.max(c);
                sum += c;
            }
            ans[i as usize] = sum - max;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_operations_queries() {
        assert_eq!(
            Solution::min_operations_queries(
                7,
                vec_vec![[0, 1, 1], [1, 2, 1], [2, 3, 1], [3, 4, 2], [4, 5, 2], [5, 6, 2]],
                vec_vec![[0, 3], [3, 6], [2, 6], [0, 6]]
            ),
            vec![0, 0, 1, 3]
        );
        assert_eq!(
            Solution::min_operations_queries(
                8,
                vec_vec![
                    [1, 2, 6],
                    [1, 3, 4],
                    [2, 4, 6],
                    [2, 5, 3],
                    [3, 6, 6],
                    [3, 0, 8],
                    [7, 0, 2]
                ],
                vec_vec![[4, 6], [0, 4], [6, 5], [7, 4]]
            ),
            vec![1, 2, 2, 3]
        )
    }
}
