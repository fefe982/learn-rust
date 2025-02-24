// https://leetcode.com/problems/design-memory-allocator/
// 2502. Design Memory Allocator
struct Allocator {
    v: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Allocator {
    fn new(n: i32) -> Self {
        Self { v: vec![0; n as usize] }
    }

    fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let mut len = 0;
        for i in 0..self.v.len() {
            if self.v[i] == 0 {
                len += 1;
                if len == size as usize {
                    let beg = i - (size as usize - 1);
                    for j in beg..i + 1 {
                        self.v[j] = m_id;
                    }
                    return beg as i32;
                }
            } else {
                len = 0;
            }
        }
        -1
    }

    fn free_memory(&mut self, m_id: i32) -> i32 {
        let mut cnt = 0;
        for i in 0..self.v.len() {
            if self.v[i] == m_id {
                self.v[i] = 0;
                cnt += 1;
            }
        }
        cnt
    }
}

/**
 * Your Allocator object will be instantiated and called as such: * let obj = Allocator::new(n);
 * let ret_1: i32 = obj.allocate(size, mID);
 * let ret_2: i32 = obj.free_memory(mID);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_it_works() {
        let mut obj = Allocator::new(10);
        assert_eq!(obj.allocate(1, 1), 0);
        assert_eq!(obj.allocate(1, 2), 1);
        assert_eq!(obj.allocate(1, 3), 2);
        assert_eq!(obj.free_memory(2), 1);
        assert_eq!(obj.allocate(3, 4), 3);
        assert_eq!(obj.allocate(1, 1), 1);
        assert_eq!(obj.allocate(1, 1), 6);
        assert_eq!(obj.free_memory(1), 3);
        assert_eq!(obj.allocate(10, 2), -1);
        assert_eq!(obj.free_memory(7), 0);
    }
}
