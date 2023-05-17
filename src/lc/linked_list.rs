// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut node = &mut head;
        for i in vec {
            node.next = Some(Box::new(ListNode::new(i)));
            node = node.next.as_mut().unwrap().as_mut();
        }
        head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_vec() {
        assert_eq!(
            ListNode::from_vec(vec![1, 2, 3, 4]),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 4, next: None }))
                    }))
                }))
            }))
        );
    }
}
