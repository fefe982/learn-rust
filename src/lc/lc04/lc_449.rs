// https://leetcode.com/problems/serialize-and-deserialize-bst/
// 449. Serialize and Deserialize BST
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    pub fn new() -> Self {
        Self {}
    }

    fn serialize_to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        if let Some(node) = root {
            res.push(node.borrow().val);
            res.append(&mut Self::serialize_to_vec(node.borrow().left.clone()));
            res.append(&mut Self::serialize_to_vec(node.borrow().right.clone()));
        }
        res
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        Self::serialize_to_vec(root)
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn deserialize_from_vec(vals: &Vec<i32>, iroot: usize, limit: i32) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
        if iroot >= vals.len() || vals[iroot] > limit {
            return (None, iroot);
        }
        let root_val = vals[iroot];
        let mut root = TreeNode::new(root_val);
        let (left, rroot) = Self::deserialize_from_vec(vals, iroot + 1, root_val);
        let (right, inext) = Self::deserialize_from_vec(vals, rroot, limit);
        root.left = left;
        root.right = right;
        (Some(Rc::new(RefCell::new(root))), inext)
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }
        let vals = data.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        Self::deserialize_from_vec(&vals, 0, i32::MAX).0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_codec() {
        let codec = Codec::new();
        assert_eq!(
            codec.deserialize(codec.serialize(TreeNode::from_vec(vec![2, 1, 3]))),
            TreeNode::from_vec(vec![2, 1, 3])
        );
        assert_eq!(
            codec.deserialize(codec.serialize(TreeNode::from_vec(vec![]))),
            TreeNode::from_vec(vec![])
        );
    }
}
