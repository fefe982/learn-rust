// https://leetcode.com/problems/design-front-middle-back-queue/
// 1670. Design Front Middle Back Queue
pub struct FrontMiddleBackQueue {
    front: std::collections::VecDeque<i32>,
    back: std::collections::VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrontMiddleBackQueue {
    pub fn new() -> Self {
        Self {
            front: std::collections::VecDeque::new(),
            back: std::collections::VecDeque::new(),
        }
    }

    pub fn push_front(&mut self, val: i32) {
        self.front.push_front(val);
        if self.front.len() > self.back.len() + 1 {
            self.back.push_front(self.front.pop_back().unwrap());
        }
    }

    pub fn push_middle(&mut self, val: i32) {
        if self.front.len() > self.back.len() {
            self.back.push_front(self.front.pop_back().unwrap());
        }
        self.front.push_back(val);
    }

    pub fn push_back(&mut self, val: i32) {
        self.back.push_back(val);
        if self.back.len() > self.front.len() {
            self.front.push_back(self.back.pop_front().unwrap());
        }
    }

    pub fn pop_front(&mut self) -> i32 {
        if let Some(v) = self.front.pop_front() {
            if self.front.len() < self.back.len() {
                self.front.push_back(self.back.pop_front().unwrap());
            }
            v
        } else {
            -1
        }
    }

    pub fn pop_middle(&mut self) -> i32 {
        if let Some(v) = self.front.pop_back() {
            if self.front.len() < self.back.len() {
                self.front.push_back(self.back.pop_front().unwrap());
            }
            v
        } else {
            -1
        }
    }

    pub fn pop_back(&mut self) -> i32 {
        if self.front.len() > self.back.len() {
            self.back.push_front(self.front.pop_back().unwrap());
        }
        self.back.pop_back().unwrap_or(-1)
    }
}

/**
 * Your FrontMiddleBackQueue object will be instantiated and called as such:
 * let obj = FrontMiddleBackQueue::new();
 * obj.push_front(val);
 * obj.push_middle(val);
 * obj.push_back(val);
 * let ret_4: i32 = obj.pop_front();
 * let ret_5: i32 = obj.pop_middle();
 * let ret_6: i32 = obj.pop_back();
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_queue() {
        let mut q = FrontMiddleBackQueue::new();
        q.push_front(1); // [1]
        q.push_back(2); // [1, 2]
        q.push_middle(3); // [1, 3, 2]
        q.push_middle(4); // [1, 4, 3, 2]
        assert_eq!(q.pop_front(), 1); // return 1 -> [4, 3, 2]
        assert_eq!(q.pop_middle(), 3); // return 3 -> [4, 2]
        assert_eq!(q.pop_middle(), 4); // return 4 -> [2]
        assert_eq!(q.pop_back(), 2); // return 2 -> []
        assert_eq!(q.pop_front(), -1); // return -1 -> [] (The queue is empty)
    }
}
