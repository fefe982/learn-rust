// https://leetcode.com/problems/maximum-number-of-k-divisible-components/
// 2872. Maximum Number of K-Divisible Components
pub struct Solution;
impl Solution {
    fn split(graph: &Vec<Vec<usize>>, values: &Vec<i32>, p: usize, n: usize, k: i32) -> (i32, i32) {
        let mut val = values[n] % k;
        let mut cnt = 0;
        for &next in &graph[n] {
            if next == p {
                continue;
            }
            let (nv, nc) = Self::split(graph, values, n, next, k);
            cnt += nc;
            val = (val + nv) % k;
        }
        if val == 0 {
            cnt += 1;
        }
        (val, cnt)
    }
    pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
        let mut graph = vec![vec![]; n as usize];
        for edge in edges {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);
        }
        Self::split(&graph, &values, usize::MAX, 0, k).1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_k_divisible_components() {
        assert_eq!(
            Solution::max_k_divisible_components(5, vec_vec![[0, 2], [1, 2], [1, 3], [2, 4]], vec![1, 8, 1, 4, 4], 6),
            2
        );
        assert_eq!(
            Solution::max_k_divisible_components(
                7,
                vec_vec![[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]],
                vec![3, 0, 6, 1, 5, 2, 1],
                3
            ),
            3
        );
    }
}
