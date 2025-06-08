// https://leetcode.com/problems/design-linked-list/
// 707. Design Linked List
pub struct MyLinkedList {
    val: i32,
    next: Option<Box<MyLinkedList>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    pub fn new() -> Self {
        Self { val: -1, next: None }
    }

    pub fn get(&self, index: i32) -> i32 {
        let mut i = index;
        let mut n = &self.next;
        while i >= 0 {
            if let Some(node) = n {
                if i == 0 {
                    return node.val;
                }
                i -= 1;
                n = &node.next;
            } else {
                return -1;
            }
        }
        return -1;
    }

    pub fn add_at_head(&mut self, val: i32) {
        let tail = self.next.take();
        self.next = Some(Box::new(MyLinkedList { val, next: tail }));
    }

    pub fn add_at_tail(&mut self, val: i32) {
        let mut n = self;
        while n.next.is_some() {
            n = n.next.as_mut().unwrap();
        }
        n.next = Some(Box::new(MyLinkedList { val, next: None }));
    }

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        let mut i = index;
        let mut n = self;
        while i >= 0 {
            if i == 0 {
                let tail = n.next.take();
                n.next = Some(Box::new(MyLinkedList { val, next: tail }));
                return;
            }
            if let Some(node) = n.next.as_mut() {
                n = node;
                i -= 1;
            } else {
                return;
            }
        }
    }

    pub fn delete_at_index(&mut self, index: i32) {
        let mut i = index;
        let mut n = self;
        while i >= 0 {
            if i == 0 {
                let tail = n.next.take();
                if let Some(node) = tail {
                    n.next = node.next;
                }
                return;
            }
            if let Some(node) = n.next.as_mut() {
                n = node;
                i -= 1;
            } else {
                return;
            }
        }
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn linked_list() {
        let mut obj = MyLinkedList::new();
        obj.add_at_head(1);
        obj.add_at_tail(3);
        obj.add_at_index(1, 2);
        assert_eq!(obj.get(1), 2);
        obj.delete_at_index(1);
        assert_eq!(obj.get(1), 3);
    }
}
