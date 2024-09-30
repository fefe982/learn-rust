// https://leetcode.com/problems/design-a-stack-with-increment-operation/
// 1381. Design a Stack With Increment Operation
struct CustomStack {
    v: Vec<i32>,
    inc: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {
    fn new(max_size: i32) -> Self {
        Self {
            v: Vec::with_capacity(max_size as usize),
            inc: vec![0; max_size as usize + 1],
        }
    }

    fn put_inc(&mut self, mut i: usize, v: i32) {
        while i > 0 {
            self.inc[i] += v;
            i -= i & (!i + 1);
        }
    }
    fn get_inc(&self, mut i: usize) -> i32 {
        let mut v = 0;
        while i < self.inc.len() {
            v += self.inc[i];
            i += i & (!i + 1);
        }
        v
    }

    fn push(&mut self, x: i32) {
        if self.v.len() + 1 >= self.inc.len() {
            return;
        }
        self.v.push(x - self.get_inc(self.v.len() + 1));
    }

    fn pop(&mut self) -> i32 {
        if let Some(v) = self.v.pop() {
            v + self.get_inc(self.v.len() + 1)
        } else {
            -1
        }
    }

    fn increment(&mut self, k: i32, val: i32) {
        self.put_inc((k as usize).min(self.v.len()), val);
    }
}

/**
 * Your CustomStack object will be instantiated and called as such:
 * let obj = CustomStack::new(maxSize);
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * obj.increment(k, val);
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_custom_stack() {
        let null = 0;
        for test in [(
            [
                "CustomStack",
                "push",
                "push",
                "pop",
                "push",
                "push",
                "push",
                "increment",
                "increment",
                "pop",
                "pop",
                "pop",
                "pop",
            ],
            vec_any![[3], [1], [2], [], [2], [3], [4], [5, 100], [2, 100], [], [], [], []],
            [null, null, null, 2, null, null, null, null, null, 103, 202, 201, -1],
        )] {
            let mut stack = CustomStack::new(test.1[0][0].as_i32());
            for ((op, args), expect) in test.0.into_iter().zip(test.1).zip(test.2).skip(1) {
                match op {
                    "push" => stack.push(args[0].as_i32()),
                    "pop" => assert_eq!(stack.pop(), expect),
                    "increment" => stack.increment(args[0].as_i32(), args[1].as_i32()),
                    _ => unreachable!(),
                }
            }
        }
    }
}
