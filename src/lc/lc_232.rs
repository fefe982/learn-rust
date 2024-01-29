// https://leetcode.com/problems/implement-queue-using-stacks/
// 232. Implement Queue using Stacks
struct MyQueue {
    q_push: Vec<i32>,
    q_pop: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Self {
            q_push: vec![],
            q_pop: vec![],
        }
    }

    fn rev(q1: &mut Vec<i32>, q2: &mut Vec<i32>) {
        while let Some(x) = q1.pop() {
            q2.push(x);
        }
    }

    fn push(&mut self, x: i32) {
        Self::rev(&mut self.q_pop, &mut self.q_push);
        self.q_push.push(x);
    }

    fn pop(&mut self) -> i32 {
        Self::rev(&mut self.q_push, &mut self.q_pop);
        self.q_pop.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        Self::rev(&mut self.q_push, &mut self.q_pop);
        self.q_pop.last().unwrap().to_owned()
    }

    fn empty(&self) -> bool {
        self.q_pop.is_empty() && self.q_push.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut obj = MyQueue::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.peek(), 1);
        assert_eq!(obj.pop(), 1);
        assert_eq!(obj.empty(), false);
    }
}
