// https://leetcode.com/problems/path-sum-iii/
// 437. Path Sum III
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn walk(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i64,
        sum: i64,
        record: &mut std::collections::HashMap<i64, i32>,
    ) -> i32 {
        if let Some(node) = root {
            let mut cnt = 0;
            let node = node.borrow();
            let sum = sum + node.val as i64;
            cnt += record.get(&(sum - target_sum)).unwrap_or(&0);
            *record.entry(sum).or_insert(0) += 1;
            cnt += Self::walk(node.left.clone(), target_sum, sum, record);
            cnt += Self::walk(node.right.clone(), target_sum, sum, record);
            *record.get_mut(&sum).unwrap() -= 1;
            cnt
        } else {
            0
        }
    }
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut record = std::collections::HashMap::new();
        record.insert(0, 1);
        Self::walk(root, target_sum as i64, 0, &mut record)
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn path_sum() {
        let null = NULL;
        assert_eq!(
            Solution::path_sum(TreeNode::from_vec(vec![10, 5, -3, 3, 2, null, 11, 3, -2, null, 1]), 8),
            3
        );
        assert_eq!(
            Solution::path_sum(
                TreeNode::from_vec(vec![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1]),
                22
            ),
            3
        );
    }
}
