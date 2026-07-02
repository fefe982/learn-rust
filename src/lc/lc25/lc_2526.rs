// https://leetcode.com/problems/find-consecutive-integers-from-a-data-stream/
// 2526. Find Consecutive Integers from a Data Stream
struct DataStream {
    value: i32,
    k: i32,
    count: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DataStream {
    fn new(value: i32, k: i32) -> Self {
        Self { value, k, count: 0 }
    }

    fn consec(&mut self, num: i32) -> bool {
        if num == self.value {
            self.count += 1;
            self.count >= self.k
        } else {
            self.count = 0;
            false
        }
    }
}

/**
 * Your DataStream object will be instantiated and called as such:
 * let obj = DataStream::new(value, k);
 * let ret_1: bool = obj.consec(num);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_data_stream() {
        let mut obj = DataStream::new(4, 3);
        assert_eq!(obj.consec(4), false);
        assert_eq!(obj.consec(4), false);
        assert_eq!(obj.consec(4), true);
        assert_eq!(obj.consec(3), false);
    }
}
