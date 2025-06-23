// https://leetcode.com/problems/count-integers-in-intervals/
// 2276. Count Integers in Intervals
pub struct CountIntervals {
    map: std::collections::BTreeMap<i32, i32>,
    cnt: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CountIntervals {
    pub fn new() -> Self {
        Self {
            map: std::collections::BTreeMap::new(),
            cnt: 0,
        }
    }

    pub fn add(&mut self, left: i32, right: i32) {
        let mut remove = vec![];
        let mut left = left;
        let mut right = right;
        for (&r, &l) in self.map.range(left..) {
            if l <= right {
                left = left.min(l);
                right = right.max(r);
                remove.push(r);
            } else {
                break;
            }
        }
        for r in remove {
            self.cnt -= r - self.map.remove(&r).unwrap() + 1;
        }
        self.map.insert(right, left);
        self.cnt += right - left + 1;
    }

    pub fn count(&self) -> i32 {
        self.cnt
    }
}

/**
 * Your CountIntervals object will be instantiated and called as such:
 * let obj = CountIntervals::new();
 * obj.add(left, right);
 * let ret_2: i32 = obj.count();
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut obj = CountIntervals::new();
        obj.add(2, 3);
        obj.add(7, 10);
        assert_eq!(obj.count(), 6);
        obj.add(5, 8);
        assert_eq!(obj.count(), 8);
    }
}
