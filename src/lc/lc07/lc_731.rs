// https://leetcode.com/problems/my-calendar-ii/
// 731. My Calendar II
pub struct MyCalendarTwo {
    book: std::collections::BTreeMap<i32, (i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {
    pub fn new() -> Self {
        Self {
            book: std::collections::BTreeMap::new(),
        }
    }

    pub fn book(&mut self, start: i32, end: i32) -> bool {
        let mut remove = vec![];
        let mut insert = vec![];
        let mut inc = vec![];
        let mut start = start;
        for (&e, &(s, cnt)) in self.book.range(start + 1..) {
            if start == end {
                break;
            }
            if s < start {
                if cnt > 0 {
                    return false;
                }
                remove.push(e);
                insert.push((s, start, cnt));
                insert.push((start, end.min(e), cnt + 1));
                start = end.min(e);
                if start < e {
                    insert.push((start, e, cnt));
                }
                continue;
            }
            if s > start {
                insert.push((start, end.min(s), 0));
                start = end.min(s);
            }
            if end <= s {
                break;
            }
            if cnt > 0 {
                return false;
            }
            if end < e {
                insert.push((start, end, cnt + 1));
                insert.push((end, e, cnt));
                remove.push(e);
                start = end;
            } else {
                inc.push(e);
                start = e;
            }
        }
        for r in remove {
            self.book.remove(&r);
        }
        for (s, e, cnt) in insert {
            self.book.insert(e, (s, cnt));
        }
        for i in inc {
            self.book.get_mut(&i).unwrap().1 += 1;
        }
        if start < end {
            self.book.insert(end, (start, 0));
        }
        true
    }
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * let obj = MyCalendarTwo::new();
 * let ret_1: bool = obj.book(start, end);
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_my_calender_ii() {
        for (book, res) in [
            (
                vec_vec![
                    [26, 35],
                    [26, 32],
                    [25, 32],
                    [18, 26],
                    [40, 45],
                    [19, 26],
                    [48, 50],
                    [1, 6],
                    [46, 50],
                    [11, 18]
                ],
                vec![true, true, false, true, true, true, true, true, true, true],
            ),
            (
                vec_vec![[10, 20], [50, 60], [10, 40], [5, 15], [5, 10], [25, 55]],
                vec![true, true, true, false, true, true],
            ),
        ] {
            let mut obj = MyCalendarTwo::new();
            for (b, r) in book.into_iter().zip(res) {
                assert_eq!(obj.book(b[0], b[1]), r);
            }
        }
    }
}
