// https://leetcode.com/problems/online-majority-element-in-subarray/
// 1157. Online Majority Element In Subarray
use std::collections::HashMap;
pub struct MajorityChecker {
    major: Vec<(i32, i32)>,
    idx_map: HashMap<i32, Vec<usize>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MajorityChecker {
    fn merge(v1: (i32, i32), v2: (i32, i32)) -> (i32, i32) {
        if v1.0 == v2.0 {
            (v1.0, v1.1 + v2.1)
        } else if v1.1 >= v2.1 {
            (v1.0, v1.1 - v2.1)
        } else {
            (v2.0, v2.1 - v1.1)
        }
    }
    pub fn new(arr: Vec<i32>) -> Self {
        let len = arr.len();
        let mut sz = 1;
        while sz < len {
            sz = sz << 2;
        }
        let mut major = vec![(0, 0); sz * 2];
        let mut idx_map: HashMap<i32, Vec<usize>> = HashMap::new();
        for (idx, &val) in arr.iter().enumerate() {
            major[sz + idx] = (val, 1);
            idx_map.entry(val).or_default().push(idx);
        }
        for idx in (1..sz).rev() {
            major[idx] = Self::merge(major[idx * 2], major[idx * 2 + 1])
        }
        Self { major, idx_map }
    }

    fn query_range(&self, idx: usize, l: i32, r: i32, left: i32, right: i32) -> (i32, i32) {
        if l >= left && r <= right {
            self.major[idx]
        } else if l >= right || r <= left {
            (0, 0)
        } else {
            let len = (r - l) / 2;
            Self::merge(
                self.query_range(idx * 2, l, l + len, left, right),
                self.query_range(idx * 2 + 1, l + len, r, left, right),
            )
        }
    }

    fn count(&self, val: i32, left: usize, right: usize) -> i32 {
        let v = self.idx_map.get(&val).unwrap();
        v.partition_point(|&x| x < right) as i32 - v.partition_point(|&x| x < left) as i32
    }

    pub fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        let possible = self
            .query_range(1, 0, self.major.len() as i32 / 2, left, right + 1)
            .0;
        if possible == 0 {
            return -1;
        }
        let cnt = self.count(possible, left as usize, right as usize + 1);
        if cnt >= threshold {
            possible
        } else {
            -1
        }
    }
}

/*
 * Your MajorityChecker object will be instantiated and called as such:
 * let obj = MajorityChecker::new(arr);
 * let ret_1: i32 = obj.query(left, right, threshold);
 */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn majority_chcker() {
        let mc = MajorityChecker::new([1, 1, 2, 2, 1, 1].to_vec());
        assert_eq!(mc.query(0, 5, 4), 1);
        assert_eq!(mc.query(0, 3, 3), -1);
        assert_eq!(mc.query(2, 3, 2), 2);
    }
}
