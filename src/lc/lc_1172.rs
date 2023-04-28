// https://leetcode.com/problems/dinner-plate-stacks/
// 1172. Dinner Plate Stacks
#[derive(PartialEq, Eq, Copy, Clone)]
struct Sz(usize);
impl PartialOrd for Sz {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}
impl Ord for Sz {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}
struct DinnerPlates {
    heap: std::collections::binary_heap::BinaryHeap<Sz>,
    content: Vec<Vec<i32>>,
    cap: usize,
    last: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        Self {
            heap: std::collections::binary_heap::BinaryHeap::new(),
            content: vec![],
            cap: capacity as usize,
            last: 0,
        }
    }

    fn push(&mut self, val: i32) {
        if let Some(&idx) = self.heap.peek() {
            let idx = idx.0;
            self.content[idx].push(val);
            if self.content[idx].len() >= self.cap {
                self.heap.pop();
            }
            if idx > self.last {
                self.last = idx;
            }
        } else {
            self.content.push(vec![val]);
            self.last = self.content.len() - 1;
            if self.cap > 1 {
                self.heap.push(Sz(self.last));
            }
        }
    }

    fn pop(&mut self) -> i32 {
        if self.last >= self.content.len() || self.content[self.last].len() == 0 {
            -1
        } else {
            if self.content[self.last].len() == self.cap {
                self.heap.push(Sz(self.last));
            }
            let val = self.content[self.last].pop().unwrap();
            while self.last > 0 && self.content[self.last].len() == 0 {
                self.last -= 1;
            }
            val
        }
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let idx = index as usize;
        if idx >= self.content.len() || self.content[idx].len() == 0 {
            -1
        } else {
            if self.content[idx].len() == self.cap {
                self.heap.push(Sz(idx));
            }
            let val = self.content[idx].pop().unwrap();
            while self.last > 0 && self.content[self.last].len() == 0 {
                self.last -= 1;
            }
            val
        }
    }
}

/*
 * Your DinnerPlates object will be instantiated and called as such:
 * let obj = DinnerPlates::new(capacity);
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.pop_at_stack(index);
 */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dinner_plates() {
        let mut d = DinnerPlates::new(2);
        d.push(1);
        d.push(2);
        d.push(3);
        d.push(4);
        d.push(5);
        assert_eq!(d.pop_at_stack(0), 2);
        d.push(20);
        d.push(21);
        assert_eq!(d.pop_at_stack(0), 20);
        assert_eq!(d.pop_at_stack(2), 21);
        assert_eq!(d.pop(), 5);
        assert_eq!(d.pop(), 4);
        assert_eq!(d.pop(), 3);
        assert_eq!(d.pop(), 1);
        assert_eq!(d.pop(), -1);
    }
}
