// https://leetcode.com/problems/find-median-from-data-stream/
// 295. Find Median from Data Stream
pub struct MedianFinder {
    high: std::collections::BinaryHeap<i32>,
    low: std::collections::BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    pub fn new() -> Self {
        Self {
            high: std::collections::BinaryHeap::new(),
            low: std::collections::BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        if self.high.len() == 0 {
            self.high.push(-num);
            return;
        }
        if num > -*self.high.peek().unwrap() {
            if self.high.len() > self.low.len() {
                self.low.push(-self.high.pop().unwrap());
            }
            self.high.push(-num);
        } else if self.high.len() > self.low.len() {
            self.low.push(num);
        } else if num < *self.low.peek().unwrap() {
            if self.low.len() > self.high.len() {
                self.high.push(-self.low.pop().unwrap());
            }
            self.low.push(num);
        } else {
            self.high.push(-num);
        }
    }

    pub fn find_median(&self) -> f64 {
        if self.high.len() > self.low.len() {
            -*self.high.peek().unwrap() as f64
        } else if self.low.len() > self.high.len() {
            *self.low.peek().unwrap() as f64
        } else {
            (-*self.high.peek().unwrap() as f64 + *self.low.peek().unwrap() as f64) / 2.0
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn median() {
        let mut m = MedianFinder::new();
        m.add_num(1);
        m.add_num(2);
        assert_eq!(m.find_median(), 1.5);
        m.add_num(3);
        assert_eq!(m.find_median(), 2.0);
    }
}
