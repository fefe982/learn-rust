// https://leetcode.com/problems/design-circular-queue/
// 622. Design Circular Queue
pub struct MyCircularQueue {
    v: Vec<i32>,
    head: usize,
    tail: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        Self {
            v: vec![0; k as usize + 1],
            head: 0,
            tail: 0,
        }
    }

    pub fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.v[self.tail] = value;
            self.tail = (self.tail + 1) % self.v.len();
            true
        }
    }

    pub fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.head = (self.head + 1) % self.v.len();
            true
        }
    }

    pub fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.v[self.head]
        }
    }

    pub fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.v[(self.tail + self.v.len() - 1) % self.v.len()]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    pub fn is_full(&self) -> bool {
        self.head == (self.tail + 1) % self.v.len()
    }
}

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_circular_queue() {
        let null = 0;
        for case in [(
            vec![
                "MyCircularQueue",
                "enQueue",
                "enQueue",
                "enQueue",
                "enQueue",
                "Rear",
                "isFull",
                "deQueue",
                "enQueue",
                "Rear",
            ],
            vec_any![[3], [1], [2], [3], [4], [], [], [], [4], []],
            vec_any![null, true, true, true, false, 3, true, true, true, 4],
        )] {
            let mut obj = MyCircularQueue::new(case.1[0][0].as_i32());
            for (i, &op) in case.0.iter().enumerate().skip(1) {
                match op {
                    "enQueue" => assert_eq!(obj.en_queue(case.1[i][0].as_i32()), case.2[i].as_bool()),
                    "deQueue" => assert_eq!(obj.de_queue(), case.2[i].as_bool()),
                    "Front" => assert_eq!(obj.front(), case.2[i].as_i32()),
                    "Rear" => assert_eq!(obj.rear(), case.2[i].as_i32()),
                    "isFull" => assert_eq!(obj.is_full(), case.2[i].as_bool()),
                    "is_empty" => assert_eq!(obj.is_empty(), case.2[i].as_bool()),
                    _ => {}
                }
            }
        }
    }
}
