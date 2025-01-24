// https://leetcode.com/problems/find-eventual-safe-states/
// 802. Find Eventual Safe States
pub struct Solution;
impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut degree = vec![0; graph.len()];
        let mut edge_in = vec![vec![]; graph.len()];
        let mut q = std::collections::VecDeque::new();
        let mut safe = vec![];
        for (i, e) in graph.iter().enumerate() {
            for &t in e {
                edge_in[t as usize].push(i);
            }
            if e.is_empty() {
                q.push_back(i);
            }
        }
        while let Some(i) = q.pop_back() {
            safe.push(i as i32);
            for &from in edge_in[i].iter() {
                let from = from as usize;
                degree[from] += 1;
                if degree[from] == graph[from].len() {
                    q.push_back(from);
                }
            }
        }
        safe.sort_unstable();
        safe
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn eventual_safe_nodes() {
        assert_eq!(
            Solution::eventual_safe_nodes(vec_vec![[1, 2], [2, 3], [5], [0], [5], [], []]),
            vec![2, 4, 5, 6]
        );
        assert_eq!(
            Solution::eventual_safe_nodes(vec_vec![[1, 2, 3, 4], [1, 2], [3, 4], [0, 4], []]),
            vec![4]
        );
    }
}
