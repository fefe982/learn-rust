// https://leetcode.com/problems/step-by-step-directions-from-a-binary-tree-node-to-another/
// 2096. Step-By-Step Directions From a Binary Tree Node to Another
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn get_path(
        root: Option<Rc<RefCell<TreeNode>>>,
        src: i32,
        dst: i32,
    ) -> (Option<String>, Option<String>, Option<String>) {
        if let Some(node) = root {
            let (ls, lup, ldown) = Solution::get_path(node.borrow_mut().left.take(), src, dst);
            if ls.is_some() {
                return (ls, None, None);
            }
            let (rs, rup, rdown) = Solution::get_path(node.borrow_mut().right.take(), src, dst);
            if rs.is_some() {
                return (rs, None, None);
            }
            let val = node.borrow().val;
            let up = if val == src {
                Some("".to_string())
            } else if lup.is_some() {
                Some(format!("U{}", lup.unwrap()))
            } else if rup.is_some() {
                Some(format!("U{}", rup.unwrap()))
            } else {
                None
            };
            let down = if val == dst {
                Some("".to_string())
            } else if ldown.is_some() {
                Some(format!("L{}", ldown.unwrap()))
            } else if rdown.is_some() {
                Some(format!("R{}", rdown.unwrap()))
            } else {
                None
            };
            if up.is_some() && down.is_some() {
                (Some(format!("{}{}", up.unwrap(), down.unwrap())), None, None)
            } else {
                (None, up, down)
            }
        } else {
            (None, None, None)
        }
    }
    pub fn get_directions(root: Option<Rc<RefCell<TreeNode>>>, start_value: i32, dest_value: i32) -> String {
        Solution::get_path(root, start_value, dest_value).0.unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_get_directions() {
        let null = NULL;
        assert_eq!(
            Solution::get_directions(TreeNode::from_vec(vec![5, 1, 2, 3, null, 6, 4]), 3, 6),
            "UURL"
        );
        assert_eq!(Solution::get_directions(TreeNode::from_vec(vec![2, 1]), 2, 1), "L");
    }
}
