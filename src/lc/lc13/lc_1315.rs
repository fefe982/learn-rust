// https://leetcode.com/problems/sum-of-nodes-with-even-valued-grandparent/
// 1315. Sum of Nodes with Even-Valued Grandparent
pub struct Solution;
use super::super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn sum_even_grandparent_(root: Option<Rc<RefCell<TreeNode>>>, grand_parent: bool, parent: bool) -> i32 {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            let val = node.val;
            let mut sum = 0;
            if grand_parent {
                sum += node.val;
            }
            sum += Self::sum_even_grandparent_(node.left.take(), parent, val % 2 == 0);
            sum += Self::sum_even_grandparent_(node.right.take(), parent, val % 2 == 0);
            sum
        } else {
            0
        }
    }
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum_even_grandparent_(root, false, false)
    }
}
#[cfg(test)]
mod test {
    use super::super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn sum_even_grandparent() {
        let null = NULL;
        assert_eq!(
            Solution::sum_even_grandparent(TreeNode::from_vec(vec![
                6, 7, 8, 2, 7, 1, 3, 9, null, 1, 4, null, null, null, 5
            ])),
            18
        );
        assert_eq!(Solution::sum_even_grandparent(TreeNode::from_vec(vec![1])), 0)
    }
}
