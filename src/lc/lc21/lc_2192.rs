// https://leetcode.com/problems/all-ancestors-of-a-node-in-a-directed-acyclic-graph/
// 2192. All Ancestors of a Node in a Directed Acyclic Graph
pub struct Solution;
impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut fwd = vec![std::collections::HashSet::<usize>::new(); n];
        let mut bwd = vec![std::collections::HashSet::<usize>::new(); n];
        for e in edges {
            fwd[e[0] as usize].insert(e[1] as usize);
            bwd[e[1] as usize].insert(e[0] as usize);
        }
        let mut ans = vec![std::collections::BTreeSet::<usize>::new(); n];
        let mut q = vec![];
        for i in 0..n {
            if bwd[i].is_empty() {
                q.push(i);
            }
        }
        while let Some(i) = q.pop() {
            for &j in fwd[i].iter() {
                ans[j].insert(i);
                let mut clone = ans[i].clone();
                ans[j].append(&mut clone);
                bwd[j].remove(&i);
                if bwd[j].is_empty() {
                    q.push(j);
                }
            }
        }
        ans.into_iter()
            .map(|a| a.into_iter().map(|x| x as i32).collect())
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_get_ancestors() {
        assert_eq!(
            Solution::get_ancestors(
                8,
                vec_vec![[0, 3], [0, 4], [1, 3], [2, 4], [2, 7], [3, 5], [3, 6], [3, 7], [4, 6]]
            ),
            vec_vec![[], [], [], [0, 1], [0, 2], [0, 1, 3], [0, 1, 2, 3, 4], [0, 1, 2, 3]]
        );
        assert_eq!(
            Solution::get_ancestors(
                5,
                vec_vec![
                    [0, 1],
                    [0, 2],
                    [0, 3],
                    [0, 4],
                    [1, 2],
                    [1, 3],
                    [1, 4],
                    [2, 3],
                    [2, 4],
                    [3, 4]
                ]
            ),
            vec_vec![[], [0], [0, 1], [0, 1, 2], [0, 1, 2, 3]]
        );
    }
}
