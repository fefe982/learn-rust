// https://leetcode.com/problems/range-module/
// 715. Range Module
pub struct RangeModule {
    map: std::collections::BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeModule {
    pub fn new() -> Self {
        Self {
            map: std::collections::BTreeMap::new(),
        }
    }

    pub fn add_range(&mut self, left: i32, right: i32) {
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
            self.map.remove(&r);
        }
        self.map.insert(right, left);
    }

    pub fn query_range(&self, left: i32, right: i32) -> bool {
        if let Some((&r, &l)) = self.map.range(left..).next() {
            l <= left && r >= right
        } else {
            false
        }
    }

    pub fn remove_range(&mut self, left: i32, right: i32) {
        let mut remove = vec![];
        let mut add = (-1, -1);
        for (r, l) in self.map.range_mut(left + 1..) {
            if *l < left {
                add = (*l, left);
            } else if *l >= right {
                break;
            }
            if *r <= right {
                remove.push(*r);
            } else {
                *l = right;
                break;
            }
        }
        for r in remove {
            self.map.remove(&r);
        }
        if add != (-1, -1) {
            self.map.insert(add.1, add.0);
        }
    }
}

/**
 * Your RangeModule object will be instantiated and called as such:
 * let obj = RangeModule::new();
 * obj.add_range(left, right);
 * let ret_2: bool = obj.query_range(left, right);
 * obj.remove_range(left, right);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_range_module() {
        let mut obj = RangeModule::new();
        obj.add_range(10, 20);
        obj.remove_range(14, 16);
        assert_eq!(obj.query_range(10, 14), true);
        assert_eq!(obj.query_range(13, 15), false);
        assert_eq!(obj.query_range(16, 17), true);
    }
}
