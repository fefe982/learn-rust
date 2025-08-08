// https://leetcode.com/problems/flip-binary-tree-to-match-preorder-traversal/
// 971. Flip Binary Tree To Match Preorder Traversal
pub struct Solution;
use super::super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
        fn flip(root: &Option<Rc<RefCell<TreeNode>>>, voyage: &[i32], ans: &mut Vec<i32>) -> usize {
            if let Some(root) = root {
                let node = root.borrow();
                if node.val != voyage[0] {
                    return usize::MAX;
                }
                if node.left.is_some() && node.right.is_some() && node.left.as_ref().unwrap().borrow().val != voyage[1]
                {
                    ans.push(node.val);
                    let r = flip(&node.right, &voyage[1..], ans);
                    if r == usize::MAX {
                        return r;
                    }
                    let l = flip(&root.borrow().left, &voyage[1 + r..], ans);
                    if l == usize::MAX {
                        return l;
                    }
                    return 1 + r + l;
                }
                let l = flip(&root.borrow().left, &voyage[1..], ans);
                if l == usize::MAX {
                    return l;
                }

                let r = flip(&node.right, &voyage[1 + l..], ans);
                if r == usize::MAX {
                    return r;
                }
                return 1 + r + l;
            } else {
                return 0;
            }
        }
        let mut ans = vec![];
        if flip(&root, &voyage, &mut ans) == usize::MAX {
            return vec![-1];
        } else {
            return ans;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn flip_match_voyage() {
        let null = NULL;
        assert_eq!(
            Solution::flip_match_voyage(TreeNode::from_vec(vec![1, 2]), vec![2, 1]),
            vec![-1]
        );
        assert_eq!(
            Solution::flip_match_voyage(TreeNode::from_vec(vec![1, 2, 3]), vec![1, 3, 2]),
            vec![1]
        );
        assert_eq!(
            Solution::flip_match_voyage(TreeNode::from_vec(vec![1, 2, 3]), vec![1, 2, 3]),
            vec![]
        );
    }
}
