// https://leetcode.com/problems/is-graph-bipartite/
// 785. Is Graph Bipartite?
pub struct Solution;
impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        let mut party: Vec<Option<i32>> = vec![None; n];
        let mut nodes: Vec<usize> = Vec::new();
        for i in 0..n {
            if let Some(_) = party[i] {
                continue;
            }
            nodes.push(i);
            party[i] = Some(0);
            while let Some(node) = nodes.pop() {
                let node_party = party[node].unwrap();
                for other_node in graph[node].iter() {
                    match party[*other_node as usize] {
                        Some(other_party) => {
                            if other_party == node_party {
                                return false;
                            }
                        }
                        None => {
                            party[*other_node as usize] = Some(1 - node_party);
                            nodes.push(*other_node as usize)
                        }
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_bipartite() {
        assert_eq!(
            Solution::is_bipartite(vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]]),
            false
        );
        assert_eq!(
            Solution::is_bipartite(vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]]),
            true
        )
    }
}
