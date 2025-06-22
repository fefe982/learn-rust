// https://leetcode.com/problems/implement-stack-using-queues/
// 225. Implement Stack using Queues
pub struct MyStack {
    q: std::collections::VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    pub fn new() -> Self {
        Self {
            q: std::collections::VecDeque::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.q.push_back(x);
    }

    pub fn pop(&mut self) -> i32 {
        let mut q = std::collections::VecDeque::new();
        while self.q.len() > 1 {
            q.push_back(self.q.pop_front().unwrap());
        }
        let n = self.q.pop_front().unwrap();
        self.q = q;
        n
    }

    pub fn top(&mut self) -> i32 {
        let mut q = std::collections::VecDeque::new();
        while let Some(n) = self.q.pop_front() {
            q.push_back(n);
            if self.q.is_empty() {
                self.q = q;
                return n;
            }
        }
        0
    }

    pub fn empty(&self) -> bool {
        self.q.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn que() {
        let mut obj = MyStack::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.top(), 2);
        assert_eq!(obj.pop(), 2);
        assert_eq!(obj.empty(), false);
    }
}
