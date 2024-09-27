// https://leetcode.com/problems/my-calendar-i/
// 729. My Calendar I
pub struct MyCalendar {
    book: std::collections::BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    pub fn new() -> Self {
        Self {
            book: std::collections::BTreeMap::new(),
        }
    }

    pub fn book(&mut self, start: i32, end: i32) -> bool {
        if let Some((_, &e)) = self.book.range(..end).rev().next() {
            if start < e {
                return false;
            }
        }
        self.book.insert(start, end);
        true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_my_calender() {
        let mut obj = MyCalendar::new();
        assert_eq!(obj.book(10, 20), true);
        assert_eq!(obj.book(15, 25), false);
        assert_eq!(obj.book(20, 30), true);
    }
}
