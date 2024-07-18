// https://leetcode.com/problems/number-of-good-leaf-nodes-pairs/
// 1530. Number of Good Leaf Nodes Pairs
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn count(root: &Option<Rc<RefCell<TreeNode>>>, distance: usize) -> (i32, Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            let (left, mut lefts) = Self::count(&node.left, distance);
            let (right, rights) = Self::count(&node.right, distance);
            let mut ret = left + right;
            let mut rsum = 0;
            for i in 0..distance - 1 {
                rsum += rights[i];
                ret += lefts[distance - 2 - i] * rsum;
                if i != distance - 2 {
                    lefts[distance - 2 - i] = lefts[distance - 2 - i - 1] + rights[distance - 2 - i - 1];
                }
            }
            lefts[0] = if node.left.is_none() && node.right.is_none() {
                1
            } else {
                0
            };
            (ret, lefts)
        } else {
            (0, vec![0; distance - 1])
        }
    }
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        if distance < 2 {
            return 0;
        }
        let distance = distance as usize;
        Self::count(&root, distance).0
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_count_pairs() {
        let null = NULL;
        assert_eq!(
            Solution::count_pairs(
                TreeNode::from_vec(vec![
                    15, 66, 55, 97, 60, 12, 56, null, 54, null, 49, null, 9, null, null, null, null, null, 90
                ]),
                5
            ),
            3
        );
        assert_eq!(Solution::count_pairs(TreeNode::from_vec(vec![1, 2, 3, null, 4]), 3), 1);
        assert_eq!(
            Solution::count_pairs(TreeNode::from_vec(vec![1, 2, 3, 4, 5, 6, 7]), 3),
            2
        );
    }
}
