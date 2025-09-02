// https://leetcode.com/problems/binary-tree-coloring-game/
// 1145. Binary Tree Coloring Game
pub struct Solution;
use super::super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, x: i32, cnt: i32) -> (i32, i32) {
            if let Some(n) = node {
                if n.borrow().val == x {
                    let left = dfs(&n.borrow().left, x, 0);
                    let right = dfs(&n.borrow().right, x, 0);
                    return (left.0, right.0);
                } else {
                    let left = dfs(&n.borrow().left, x, cnt);
                    let right = dfs(&n.borrow().right, x, cnt);
                    if cnt == -1 {
                        if left.0 != -1 {
                            return left;
                        } else if right.0 != -1 {
                            return right;
                        } else {
                            return (-1, -1);
                        }
                    } else {
                        return (left.0 + right.0 + 1, 0);
                    }
                }
            } else {
                if cnt == -1 {
                    return (-1, -1);
                } else {
                    return (0, 0);
                }
            }
        }
        let (left, right) = dfs(&root, x, -1);
        let parent = n - left - right - 1;
        let half = n / 2;
        left > half || right > half || parent > half
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn btree_game_winning_move() {
        assert_eq!(
            Solution::btree_game_winning_move(TreeNode::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]), 11, 3),
            true
        );
        assert_eq!(
            Solution::btree_game_winning_move(TreeNode::from_vec(vec![1, 2, 3]), 3, 1),
            false
        );
    }
}
