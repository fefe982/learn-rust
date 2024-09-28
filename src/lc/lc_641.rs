// https://leetcode.com/problems/design-circular-deque/
// 641. Design Circular Deque
struct MyCircularDeque {
    v: Vec<i32>,
    l: usize,
    r: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            v: vec![0; k as usize + 1],
            l: 0,
            r: 0,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.l = (self.l + self.v.len() - 1) % self.v.len();
        self.v[self.l] = value;
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.v[self.r] = value;
        self.r = (self.r + 1) % self.v.len();
        true
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.l = (self.l + 1) % self.v.len();
        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.r = (self.r + self.v.len() - 1) % self.v.len();
        true
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.v[self.l]
        }
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.v[(self.r + self.v.len() - 1) % self.v.len()]
        }
    }

    fn is_empty(&self) -> bool {
        self.l == self.r
    }

    fn is_full(&self) -> bool {
        (self.r + 1) % self.v.len() == self.l
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_circular_deque() {
        let null = 0;
        for test in [(
            [
                "MyCircularDeque",
                "insertLast",
                "insertLast",
                "insertFront",
                "insertFront",
                "getRear",
                "isFull",
                "deleteLast",
                "insertFront",
                "getFront",
            ],
            vec_vec![[3], [1], [2], [3], [4], [], [], [], [4], []],
            vec_any![null, true, true, true, false, 2, true, true, true, 4],
        )] {
            let mut obj = MyCircularDeque::new(test.1[0][0]);
            for ((cmd, args), expect) in test.0.into_iter().zip(test.1).zip(test.2).skip(1) {
                match cmd {
                    "insertFront" => assert_eq!(obj.insert_front(args[0]), expect.as_bool()),
                    "insertLast" => assert_eq!(obj.insert_last(args[0]), expect.as_bool()),
                    "deleteFront" => assert_eq!(obj.delete_front(), expect.as_bool()),
                    "deleteLast" => assert_eq!(obj.delete_last(), expect.as_bool()),
                    "getFront" => assert_eq!(obj.get_front(), expect.as_i32()),
                    "getRear" => assert_eq!(obj.get_rear(), expect.as_i32()),
                    "isFull" => assert_eq!(obj.is_full(), expect.as_bool()),
                    "isEmpty" => assert_eq!(obj.is_empty(), expect.as_bool()),
                    _ => unreachable!(),
                }
            }
        }
    }
}
