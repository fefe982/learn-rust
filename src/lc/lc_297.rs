// https://leetcode.com/problems/serialize-and-deserialize-binary-tree/
// 297. Serialize and Deserialize Binary Tree
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut builder = String::from("");
        let mut q = std::collections::VecDeque::<Option<Rc<RefCell<TreeNode>>>>::new();
        q.push_back(root);
        while let Some(r) = q.pop_front() {
            if let Some(n) = r {
                builder.push_str(&n.borrow().val.to_string());
                q.push_back(n.borrow().left.clone());
                q.push_back(n.borrow().right.clone());
            } else {
                builder.push('n')
            }
            builder.push(';')
        }
        builder
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let data: &str = &data;
        let pos = data.find(';').unwrap();
        if &data[0..1] == "n" {
            return None;
        }
        let root = Some(Rc::new(RefCell::new(TreeNode::new(
            data[0..pos].parse::<i32>().unwrap(),
        ))));
        let mut que = std::collections::VecDeque::new();
        que.push_back(root.clone());
        let mut s = pos + 1;
        while let Some(parent) = que.pop_front() {
            let parent_node = parent.clone().unwrap();
            let mut nopt: Option<i32> = None;
            if s < data.len() {
                let ns = data[s..].find(';').unwrap();
                if &data[s..s + 1] != "n" {
                    nopt = Some(data[s..s + ns].parse::<i32>().unwrap());
                }
                s += ns + 1;
            }
            if let Some(n) = nopt {
                let new_node = Some(Rc::new(RefCell::new(TreeNode::new(n))));
                que.push_back(new_node.clone());
                parent_node.as_ref().borrow_mut().left = new_node.clone();
            }
            nopt = None;
            if s < data.len() {
                let ns = data[s..].find(';').unwrap();
                if &data[s..s + 1] != "n" {
                    nopt = Some(data[s..s + ns].parse::<i32>().unwrap());
                }
                s += ns + 1;
            }
            if let Some(n) = nopt {
                let new_node = Some(Rc::new(RefCell::new(TreeNode::new(n))));
                que.push_back(new_node.clone());
                parent_node.as_ref().borrow_mut().right = new_node.clone();
            }
        }
        root
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn bulb_switch() {
        let c = Codec::new();
        assert_eq!(
            c.deserialize(c.serialize(TreeNode::from_vec(vec![1, 2, 3, NULL, NULL, 4, 5]))),
            TreeNode::from_vec(vec![1, 2, 3, NULL, NULL, 4, 5])
        );
        assert_eq!(
            c.deserialize(c.serialize(TreeNode::from_vec(vec![]))),
            TreeNode::from_vec(vec![])
        );
        assert_eq!(
            c.deserialize(c.serialize(TreeNode::from_vec(vec![1, 2]))),
            TreeNode::from_vec(vec![1, 2])
        );
    }
}
