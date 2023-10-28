// https://leetcode.com/problems/validate-binary-tree-nodes/
// 1361. Validate Binary Tree Nodes
pub struct Solution;
impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let n = n as usize;
        let mut visited = vec![true; n];
        for &c in left_child.iter().chain(right_child.iter()) {
            if c >= 0 {
                let c = c as usize;
                if !visited[c] {
                    return false;
                }
                visited[c] = false;
            }
        }
        let mut root = usize::MAX;
        for (i, &v) in visited.iter().enumerate() {
            if v {
                if root != usize::MAX {
                    return false;
                } else {
                    root = i;
                }
            }
        }
        if root == usize::MAX {
            return false;
        }
        visited[root] = false;
        let mut q = vec![root];
        while let Some(node) = q.pop() {
            if visited[node] {
                return false;
            }
            visited[node] = true;
            if left_child[node] != -1 {
                q.push(left_child[node] as usize);
            }
            if right_child[node] != -1 {
                q.push(right_child[node] as usize);
            }
        }
        visited.iter().all(|&v| v == true)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_validate_binary_tree_nodes() {
        assert_eq!(
            Solution::validate_binary_tree_nodes(4, vec![3, -1, 1, -1], vec![-1, -1, 0, -1]),
            true
        );
        assert_eq!(
            Solution::validate_binary_tree_nodes(4, vec![1, -1, 3, -1], vec![2, -1, -1, -1]),
            true
        );
        assert_eq!(
            Solution::validate_binary_tree_nodes(4, vec![1, -1, 3, -1], vec![2, 3, -1, -1]),
            false
        );
        assert_eq!(
            Solution::validate_binary_tree_nodes(2, vec![1, 0], vec![-1, -1]),
            false
        );
    }
}
