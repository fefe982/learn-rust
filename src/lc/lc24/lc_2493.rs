// https://leetcode.com/problems/divide-nodes-into-the-maximum-number-of-groups/
// 2493. Divide Nodes Into the Maximum Number of Groups
pub struct Solution;
impl Solution {
    pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let inf = i32::MAX / 2;
        let mut graph = vec![vec![inf; n]; n];
        for i in 0..n {
            graph[i][i] = 0;
        }
        for edge in edges {
            graph[edge[0] as usize - 1][edge[1] as usize - 1] = 1;
            graph[edge[1] as usize - 1][edge[0] as usize - 1] = 1;
        }
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    graph[j][k] = graph[j][k].min(graph[j][i] + graph[i][k]);
                }
            }
        }
        let mut visited = vec![0; n];
        let mut ans = 0;
        let mut head = vec![0; n];
        let mut max = vec![0; n];
        for i in 0..n {
            let mut h = head[i];
            if visited[i] == 0 {
                visited[i] = 1;
                head[i] = i;
                h = i;
            }
            let mut lmax = max[h];
            let mut last = true;
            for j in i + 1..n {
                if graph[i][j] < inf {
                    last = false;
                    if visited[j] == 0 {
                        visited[j] = if graph[i][j] % 2 == 1 { 2 } else { 1 };
                        head[j] = h;
                    } else {
                        if graph[i][j] % 2 != (visited[i] + visited[j]) % 2 {
                            return -1;
                        }
                    }
                    lmax = lmax.max(graph[i][j]);
                }
            }
            if last {
                ans += lmax + 1;
            } else {
                max[h] = lmax;
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
    fn test_magnificent_sets() {
        assert_eq!(Solution::magnificent_sets(30, vec_vec![[16, 8], [6, 5]]), 30);
        assert_eq!(
            Solution::magnificent_sets(6, vec_vec![[1, 2], [1, 4], [1, 5], [2, 6], [2, 3], [4, 6]]),
            4
        );
        assert_eq!(Solution::magnificent_sets(3, vec_vec![[1, 2], [2, 3], [3, 1]]), -1);
    }
}
