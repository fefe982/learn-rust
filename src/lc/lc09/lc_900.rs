// https://leetcode.com/problems/rle-iterator/
// 900. RLE Iterator
pub struct RLEIterator {
    rle: Vec<i32>,
    index: usize,
    cur: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RLEIterator {
    pub fn new(encoding: Vec<i32>) -> Self {
        Self {
            rle: encoding,
            index: 0,
            cur: 0,
        }
    }

    pub fn next(&mut self, n: i32) -> i32 {
        let mut n = n;
        while n > 0 && self.index < self.rle.len() && self.rle[self.index] - self.cur < n {
            n -= self.rle[self.index] - self.cur;
            self.index += 2;
            self.cur = 0
        }
        if self.index >= self.rle.len() {
            -1
        } else {
            self.cur += n;
            self.rle[self.index + 1]
        }
    }
}

/**
 * Your RLEIterator object will be instantiated and called as such:
 * let obj = RLEIterator::new(encoding);
 * let ret_1: i32 = obj.next(n);
 */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_900() {
        let mut obj = RLEIterator::new(vec![3, 8, 0, 9, 2, 5]);
        assert_eq!(obj.next(2), 8);
        assert_eq!(obj.next(1), 8);
        assert_eq!(obj.next(1), 5);
        assert_eq!(obj.next(2), -1);
    }
}
