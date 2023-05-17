// https://leetcode.com/problems/linked-list-random-node
// 382. Linked List Random Node

use super::linked_list::ListNode;
pub struct Solution {
    arr: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
use rand;
impl Solution {
    pub fn new(head: Option<Box<ListNode>>) -> Self {
        let mut arr: Vec<i32> = Vec::new();
        let mut head = head;
        while let Some(node) = head {
            arr.push(node.val);
            head = node.next;
        }
        Self { arr }
    }

    pub fn get_random(&self) -> i32 {
        self.arr[rand::random::<i64>() as usize % self.arr.len()]
    }
}

/*
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_random() {
        let obj = Solution::new(ListNode::from_vec(vec![1, 2, 3, 4]));
        let i = obj.get_random();
        assert_ne!(i, 0)
    }
}
