// https://leetcode.com/problems/create-binary-tree-from-descriptions/
// 2196. Create Binary Tree From Descriptions
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes = std::collections::HashMap::new();
        let mut parent_nodes = std::collections::HashSet::new();
        let mut child_nodes = std::collections::HashSet::new();
        for desc in descriptions {
            let parent = desc[0];
            let child = desc[1];
            let is_left = desc[2] == 1;
            let child_node = nodes
                .entry(child)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(child))))
                .clone();
            let parent_node = nodes
                .entry(parent)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(parent))));
            if is_left {
                parent_node.borrow_mut().left = Some(child_node);
            } else {
                parent_node.borrow_mut().right = Some(child_node);
            }
            child_nodes.insert(child);
            if parent_nodes.contains(&child) {
                parent_nodes.remove(&child);
            }
            if !child_nodes.contains(&parent) {
                parent_nodes.insert(parent);
            }
        }
        nodes.remove(parent_nodes.iter().next().unwrap())
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    use crate::*;
    #[test]
    fn test_create_binary_tree() {
        let null = NULL;
        assert_eq!(
            Solution::create_binary_tree(vec_vec![
                [20, 15, 1],
                [20, 17, 0],
                [50, 20, 1],
                [50, 80, 0],
                [80, 19, 1]
            ]),
            TreeNode::from_vec(vec![50, 20, 80, 15, 17, 19])
        );
        assert_eq!(
            Solution::create_binary_tree(vec_vec![[1, 2, 1], [2, 3, 0], [3, 4, 1]]),
            TreeNode::from_vec(vec![1, 2, null, null, 3, 4])
        );
    }
}
