// https://leetcode.com/problems/binary-tree-cameras/
// 968. Binary Tree Cameras
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
#[derive(PartialEq, Eq)]
enum State {
    CoveredWithCamera,
    CoveredWithoutCamera,
    NotCovered,
}
impl Solution {
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, cnt: &mut i32) -> State {
        if let Some(node) = root {
            let left = Self::dfs(&node.borrow().left, cnt);
            let right = Self::dfs(&node.borrow().right, cnt);
            if left == State::NotCovered || right == State::NotCovered {
                *cnt += 1;
                return State::CoveredWithCamera;
            }
            if left == State::CoveredWithCamera || right == State::CoveredWithCamera {
                return State::CoveredWithoutCamera;
            }
            return State::NotCovered;
        } else {
            State::CoveredWithoutCamera
        }
    }
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut cnt = 0;
        let state = Self::dfs(&root, &mut cnt);
        if state == State::NotCovered {
            cnt + 1
        } else {
            cnt
        }
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_min_camera_cover() {
        let null = NULL;
        assert_eq!(
            Solution::min_camera_cover(TreeNode::from_vec(vec![0, 0, null, 0, 0])),
            1
        );
        assert_eq!(
            Solution::min_camera_cover(TreeNode::from_vec(vec![0, 0, null, 0, null, 0, null, null, 0])),
            2
        );
    }
}
