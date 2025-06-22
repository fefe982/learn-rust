// https://leetcode.com/problems/min-stack/
// 155. Min Stack
pub struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.min_stack.is_empty() || val <= *self.min_stack.last().unwrap() {
            self.min_stack.push(val);
        }
    }

    pub fn pop(&mut self) {
        if self.stack.pop().unwrap() == *self.min_stack.last().unwrap() {
            self.min_stack.pop();
        }
    }

    pub fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_stack() {
        let mut obj = MinStack::new();
        obj.push(-2);
        obj.push(0);
        obj.push(-3);
        assert_eq!(obj.get_min(), -3);
        obj.pop();
        assert_eq!(obj.top(), 0);
        assert_eq!(obj.get_min(), -2);
    }
}
