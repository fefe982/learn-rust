// https://leetcode.com/problems/find-eventual-safe-states/
// 802. Find Eventual Safe States
pub struct Solution;
impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut edge_out: Vec<std::collections::HashSet<i32>> =
            graph.iter().map(|v| v.iter().cloned().collect()).collect();
        let mut edge_in = vec![std::collections::HashSet::<i32>::new(); edge_out.len()];
        let mut q = std::collections::VecDeque::new();
        let mut safe = std::collections::BTreeSet::new();
        for (e, i) in edge_out.iter().zip(0..) {
            if e.is_empty() {
                q.push_back(i);
            } else {
                for &to in e {
                    edge_in[to as usize].insert(i);
                }
            }
        }
        while let Some(i) = q.pop_back() {
            safe.insert(i);
            for &from in &edge_in[i as usize] {
                edge_out[from as usize].remove(&i);
                if edge_out[from as usize].is_empty() {
                    q.push_back(from);
                }
            }
        }
        safe.into_iter().collect()
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
