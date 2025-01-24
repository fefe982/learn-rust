// https://leetcode.com/problems/find-eventual-safe-states/
// 802. Find Eventual Safe States
pub struct Solution;
impl Solution {
    fn walk(graph: &Vec<Vec<i32>>, node: usize, state: &mut Vec<i32>) -> i32 {
        if state[node] != 0 {
            return state[node];
        }
        state[node] = 1;
        for &n in graph[node].iter() {
            if Self::walk(graph, n as usize, state) == 1 {
                return 1;
            }
        }
        state[node] = 2;
        2
    }
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut state = vec![0; graph.len()];
        for i in 0..graph.len() {
            Self::walk(&graph, i, &mut state);
        }
        state
            .into_iter()
            .enumerate()
            .filter_map(|(i, s)| if s == 2 { Some(i as i32) } else { None })
            .collect::<Vec<_>>()
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
