// https://leetcode.com/problems/number-of-recent-calls/
// 933. Number of Recent Calls
use std::collections::VecDeque;
pub struct RecentCounter {
    q: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    pub fn new() -> Self {
        Self { q: VecDeque::new() }
    }

    pub fn ping(&mut self, t: i32) -> i32 {
        self.q.push_back(t);
        while self.q.front().unwrap() < &(t - 3000) {
            self.q.pop_front();
        }
        self.q.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recent_counter() {
        let mut obj = RecentCounter::new();
        assert_eq!(obj.ping(1), 1);
        assert_eq!(obj.ping(100), 2);
        assert_eq!(obj.ping(3001), 3);
        assert_eq!(obj.ping(3002), 3);
    }
}
