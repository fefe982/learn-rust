// https://leetcode.com/problems/seat-reservation-manager/
// 1845. Seat Reservation Manager
pub struct SeatManager {
    h: std::collections::BinaryHeap<std::cmp::Reverse<i32>>,
    n: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {
    pub fn new(_n: i32) -> Self {
        Self {
            h: std::collections::BinaryHeap::new(),
            n: 1,
        }
    }

    pub fn reserve(&mut self) -> i32 {
        if let Some(n) = self.h.pop() {
            n.0
        } else {
            self.n += 1;
            self.n - 1
        }
    }

    pub fn unreserve(&mut self, seat_number: i32) {
        self.h.push(std::cmp::Reverse(seat_number));
    }
}

/**
 * Your SeatManager object will be instantiated and called as such:
 * let obj = SeatManager::new(n);
 * let ret_1: i32 = obj.reserve();
 * obj.unreserve(seatNumber);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_seet_manager() {
        let mut obj = SeatManager::new(5);
        assert_eq!(obj.reserve(), 1);
        assert_eq!(obj.reserve(), 2);
        obj.unreserve(2);
        assert_eq!(obj.reserve(), 2);
        assert_eq!(obj.reserve(), 3);
        assert_eq!(obj.reserve(), 4);
        assert_eq!(obj.reserve(), 5);
        obj.unreserve(5);
    }
}
