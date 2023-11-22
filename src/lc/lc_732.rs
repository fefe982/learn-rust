// https://leetcode.com/problems/my-calendar-iii/
// 732. My Calendar III
pub struct MyCalendarThree {
    books: std::collections::BTreeMap<i32, (i32, i32)>,
    max: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {
    pub fn new() -> Self {
        Self {
            books: std::collections::BTreeMap::new(),
            max: 0,
        }
    }

    pub fn book(&mut self, start_time: i32, end_time: i32) -> i32 {
        let mut add = vec![];
        let mut max_base = 0;
        let mut start_time = start_time;
        for (&r, (l, h)) in self.books.range_mut(start_time + 1..) {
            if *l < end_time {
                max_base = max_base.max(*h);
                if *l < start_time {
                    add.push((start_time, *l, *h));
                } else if *l > start_time {
                    add.push((*l, start_time, 1));
                    start_time = *l;
                }
                if end_time < r {
                    add.push((end_time, start_time, *h + 1));
                    *l = end_time;
                    start_time = end_time;
                } else if end_time >= r {
                    *l = start_time;
                    *h = *h + 1;
                    start_time = r;
                }
            } else {
                break;
            }
        }
        if start_time < end_time {
            add.push((end_time, start_time, 1));
        }
        for (r, l, h) in add {
            self.books.insert(r, (l, h));
        }
        self.max = self.max.max(max_base + 1);
        self.max
    }
}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(startTime, endTime);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_my_calender() {
        let mut obj = MyCalendarThree::new();
        assert_eq!(obj.book(10, 20), 1);
        assert_eq!(obj.book(50, 60), 1);
        assert_eq!(obj.book(10, 40), 2);
        assert_eq!(obj.book(5, 15), 3);
        assert_eq!(obj.book(5, 10), 3);
        assert_eq!(obj.book(25, 55), 3);
    }
}
