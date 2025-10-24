// https://leetcode.com/problems/number-of-nodes-in-the-sub-tree-with-the-same-label/
// 1519. Number of Nodes in the Sub-Tree with the Same Label
pub struct Solution;
impl Solution {
    fn count(edges: &Vec<Vec<usize>>, labels: &Vec<usize>, node: usize, parent: usize, res: &mut Vec<i32>) -> Vec<i32> {
        let label = labels[node];
        let mut count = vec![0; 26];
        count[label] += 1;
        for &child in edges[node].iter() {
            if child != parent {
                let count_child = Self::count(edges, labels, child as usize, node, res);
                res[node] += count_child[label];
                for i in 0..26 {
                    count[i] += count_child[i];
                }
            }
        }
        count
    }
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let mut e = vec![vec![]; n as usize];
        for edge in edges {
            e[edge[0] as usize].push(edge[1] as usize);
            e[edge[1] as usize].push(edge[0] as usize);
        }
        let mut res = vec![1; n as usize];
        Self::count(
            &e,
            &labels.chars().map(|c| (c as u8 - b'a') as usize).collect(),
            0,
            usize::MAX,
            &mut res,
        );
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_sub_trees() {
        assert_eq!(
            Solution::count_sub_trees(
                7,
                vec_vec![[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]],
                "abaedcd".to_string()
            ),
            vec![2, 1, 1, 1, 1, 1, 1]
        );
        assert_eq!(
            Solution::count_sub_trees(4, vec_vec![[0, 1], [1, 2], [0, 3]], "bbbb".to_string()),
            vec![4, 2, 1, 1]
        );
        assert_eq!(
            Solution::count_sub_trees(5, vec_vec![[0, 1], [0, 2], [1, 3], [0, 4]], "aabab".to_string()),
            vec![3, 2, 1, 1, 1]
        );
    }
}
