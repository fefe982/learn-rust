// https://leetcode.com/problems/modify-graph-edge-weights/
// 2699. Modify Graph Edge Weights
pub struct Solution;
impl Solution {
    fn dijkstra(
        n: usize,
        graph: &mut Vec<Vec<i32>>,
        source: usize,
        destination: usize,
        target: i32,
        first_run: &Vec<i32>,
    ) -> Vec<i32> {
        let mut fixed = vec![false; n];
        let mut res = vec![i32::MAX; n];
        res[source] = 0;
        loop {
            let mut min = i32::MAX;
            let mut min_id = 0;
            for i in 0..n {
                if fixed[i] {
                    continue;
                }
                if res[i] < min {
                    min = res[i];
                    min_id = i;
                }
            }
            if min == i32::MAX {
                return res;
            }
            fixed[min_id] = true;
            for j in 0..n {
                let mut d = graph[min_id][j];
                if d < 0 {
                    d = 1;
                    if first_run.len() > 0 {
                        let v = target - first_run[destination] + first_run[j] - res[min_id];
                        if v >= d {
                            graph[min_id][j] = v;
                            graph[j][min_id] = v;
                            d = v;
                        }
                    }
                }
                if d == i32::MAX {
                    continue;
                }
                res[j] = std::cmp::min(res[j], res[min_id] + d);
            }
        }
    }
    pub fn modified_graph_edges(
        n: i32,
        mut edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
        target: i32,
    ) -> Vec<Vec<i32>> {
        let n = n as usize;
        let source = source as usize;
        let destination = destination as usize;
        let mut graph = vec![vec![i32::MAX; n]; n];
        for e in &edges {
            graph[e[0] as usize][e[1] as usize] = e[2];
            graph[e[1] as usize][e[0] as usize] = e[2];
        }
        let first_run = Self::dijkstra(n, &mut graph, source, destination, target, &Vec::new());
        if first_run[destination] > target {
            return Vec::new();
        }
        let second_run = Self::dijkstra(n, &mut graph, source, destination, target, &first_run);
        if second_run[destination] != target {
            return Vec::new();
        }
        for e in edges.iter_mut() {
            e[2] = std::cmp::max(graph[e[0] as usize][e[1] as usize], 1);
        }
        edges
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn modified_graph_edges() {
        assert_eq!(
            Solution::modified_graph_edges(
                5,
                vec_vec![
                    [0, 2, 5],
                    [2, 1, -1],
                    [2, 4, 3],
                    [3, 4, 5],
                    [4, 0, 1],
                    [0, 3, -1],
                    [2, 3, -1]
                ],
                0,
                1,
                9
            ),
            vec_vec![
                [0, 2, 5],
                [2, 1, 5],
                [2, 4, 3],
                [3, 4, 5],
                [4, 0, 1],
                [0, 3, 7],
                [2, 3, 3]
            ]
        );
        assert_eq!(
            Solution::modified_graph_edges(
                5,
                vec_vec![
                    [0, 3, 5],
                    [2, 1, -1],
                    [1, 3, 5],
                    [3, 4, -1],
                    [4, 2, -1],
                    [0, 2, 8],
                    [4, 0, 9],
                    [2, 3, 7]
                ],
                0,
                1,
                11
            ),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(
            Solution::modified_graph_edges(
                4,
                vec_vec![[3, 0, -1], [1, 2, -1], [2, 3, -1], [1, 3, 9], [2, 0, 5]],
                0,
                1,
                7
            ),
            vec_vec![[3, 0, 5], [1, 2, 2], [2, 3, 1], [1, 3, 9], [2, 0, 5]]
        );
        assert_eq!(
            Solution::modified_graph_edges(
                5,
                vec_vec![[4, 1, -1], [2, 0, -1], [0, 3, -1], [4, 3, -1]],
                0,
                1,
                5
            ),
            vec_vec![[4, 1, 1], [2, 0, 3], [0, 3, 3], [4, 3, 1]]
        );
        assert_eq!(
            Solution::modified_graph_edges(3, vec_vec![[0, 1, -1], [0, 2, 5]], 0, 2, 6),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(
            Solution::modified_graph_edges(
                4,
                vec_vec![[1, 0, 4], [1, 2, 3], [2, 3, 5], [0, 3, -1]],
                0,
                2,
                6
            ),
            [[1, 0, 4], [1, 2, 3], [2, 3, 5], [0, 3, 1]]
        );
    }
}
