// https://leetcode.com/problems/add-edges-to-make-degrees-of-all-nodes-even/
// 2508. Add Edges to Make Degrees of All Nodes Even
pub struct Solution;
impl Solution {
    pub fn is_possible(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let mut graph = vec![std::collections::HashSet::new(); n as usize];
        for edge in edges {
            graph[edge[0] as usize - 1].insert(edge[1] as usize - 1);
            graph[edge[1] as usize - 1].insert(edge[0] as usize - 1);
        }
        let mut even = vec![];
        let mut odd = vec![];
        for (i, node) in graph.iter().enumerate() {
            if node.len() % 2 == 0 {
                even.push(i);
            } else {
                odd.push(i);
            }
        }
        match odd.len() {
            0 => true,
            2 => {
                if !graph[odd[0]].contains(&odd[1]) {
                    true
                } else {
                    let mut found = false;
                    for e in even.iter() {
                        if !graph[*e].contains(&odd[0]) && !graph[*e].contains(&odd[1]) {
                            found = true;
                            break;
                        }
                    }
                    found
                }
            }
            4 => {
                if !graph[odd[0]].contains(&odd[1]) && !graph[odd[2]].contains(&odd[3]) {
                    true
                } else if !graph[odd[0]].contains(&odd[2]) && !graph[odd[1]].contains(&odd[3]) {
                    true
                } else if !graph[odd[0]].contains(&odd[3]) && !graph[odd[1]].contains(&odd[2]) {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_possible() {
        assert_eq!(
            Solution::is_possible(5, vec_vec![[1, 2], [2, 3], [3, 4], [4, 2], [1, 4], [2, 5]]),
            true
        );
        assert_eq!(Solution::is_possible(4, vec_vec![[1, 2], [3, 4]]), true);
        assert_eq!(Solution::is_possible(4, vec_vec![[1, 2], [1, 3], [1, 4]]), false);
    }
}
