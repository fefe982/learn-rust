// https://leetcode.com/problems/difference-between-maximum-and-minimum-price-sum/
// 2538. Difference Between Maximum and Minimum Price Sum
pub struct Solution;
impl Solution {
    fn walk_diff(graph: &mut Vec<Vec<(usize, i64)>>, price: &Vec<i32>, p: usize, node: usize) -> i64 {
        let mut max_p = vec![0; 2];
        for &(_, d) in &graph[node] {
            if d > max_p[0] {
                max_p[1] = max_p[0];
                max_p[0] = d;
            } else if d > max_p[1] {
                max_p[1] = d;
            }
        }
        let mut max_diff = max_p[0];
        for i in 0..graph[node].len() {
            let (next, nd) = graph[node][i];
            if next != p {
                let np = if nd == max_p[0] { max_p[1] } else { max_p[0] };
                graph[next][0].1 = np + price[node] as i64;
                max_diff = max_diff.max(Self::walk_diff(graph, price, node, next));
            }
        }
        max_diff
    }
    fn walk(graph: &mut Vec<Vec<(usize, i64)>>, price: &Vec<i32>, p: usize, node: usize) -> i64 {
        let mut max_path = 0;
        let mut pi = 0;
        for i in 0..graph[node].len() {
            let next = graph[node][i].0;
            if next != p {
                let nextd = Self::walk(graph, price, node, next);
                graph[node][i].1 = nextd;
                max_path = max_path.max(nextd);
            } else {
                pi = i;
            }
        }
        if pi != 0 {
            graph[node].swap(0, pi);
        }
        return price[node] as i64 + max_path;
    }
    pub fn max_output(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>) -> i64 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for edge in edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            graph[a].push((b, 0));
            graph[b].push((a, 0));
        }
        Self::walk(&mut graph, &price, usize::MAX, 0);
        Self::walk_diff(&mut graph, &price, usize::MAX, 0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_output() {
        assert_eq!(
            Solution::max_output(4, vec_vec![[2, 0], [0, 1], [1, 3]], vec![2, 3, 1, 1]),
            6
        );
        assert_eq!(
            Solution::max_output(
                6,
                vec_vec![[0, 1], [1, 2], [1, 3], [3, 4], [3, 5]],
                vec![9, 8, 7, 6, 10, 5]
            ),
            24
        );
        assert_eq!(Solution::max_output(3, vec_vec![[0, 1], [1, 2]], vec![1, 1, 1]), 2);
    }
}
