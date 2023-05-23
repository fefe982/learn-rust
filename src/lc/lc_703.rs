// https://leetcode.com/problems/kth-largest-element-in-a-stream/
// 703. Kth Largest Element in a Stream
pub struct Solution;
pub struct KthLargest {
    q: std::collections::BinaryHeap<i32>,
    k: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut q = std::collections::BinaryHeap::new();
        let k = k as usize;
        for i in 0..k {
            if i < nums.len() {
                q.push(-nums[i]);
            }
        }
        let mut s = Self { q, k };
        for i in k..nums.len() {
            s.add(nums[i]);
        }
        s
    }

    pub fn add(&mut self, val: i32) -> i32 {
        if self.q.len() == self.k && -val < *self.q.peek().unwrap() {
            self.q.pop();
        }
        if self.q.len() < self.k {
            self.q.push(-val);
        }
        -*self.q.peek().unwrap()
    }
}

/*
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn kth_large() {
        let mut k = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(k.add(3), 4);
        assert_eq!(k.add(5), 5);
        assert_eq!(k.add(10), 5);
        assert_eq!(k.add(9), 8);
        assert_eq!(k.add(4), 8);
    }
}
