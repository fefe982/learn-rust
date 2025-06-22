// https://leetcode.com/problems/find-largest-value-in-each-tree-row/
// 515. Find Largest Value in Each Tree Row
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn find_largest_values(root: &Option<Rc<RefCell<TreeNode>>>, level: usize, vec: &mut Vec<i32>) {
        if let Some(node) = root {
            if level == vec.len() {
                vec.push(node.borrow().val);
            } else {
                vec[level] = vec[level].max(node.borrow().val);
            }
            Solution::find_largest_values(&node.borrow().left, level + 1, vec);
            Solution::find_largest_values(&node.borrow().right, level + 1, vec);
        }
    }
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut vec = vec![];
        Solution::find_largest_values(&root, 0, &mut vec);
        vec
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_largest_values() {
        let null = NULL;
        assert_eq!(
            Solution::largest_values(TreeNode::from_vec(vec![1, 3, 2, 5, 3, null, 9])),
            vec![1, 3, 9]
        );
        assert_eq!(
            Solution::largest_values(TreeNode::from_vec(vec![1, 2, 3])),
            vec![1, 3]
        );
    }
}
