// https://leetcode.com/problems/snapshot-array/
// 1146. Snapshot Array
pub struct SnapshotArray {
    arr: Vec<Vec<(i32, i32)>>,
    snap_id: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {
    pub fn new(length: i32) -> Self {
        Self {
            arr: vec![Vec::new(); length as usize],
            snap_id: 0,
        }
    }

    pub fn set(&mut self, index: i32, val: i32) {
        let arr = &mut self.arr[index as usize];
        match arr.last_mut() {
            Some((i, v)) if *i == self.snap_id => *v = val,
            _ => arr.push((self.snap_id, val)),
        }
    }

    pub fn snap(&mut self) -> i32 {
        let id = self.snap_id;
        self.snap_id += 1;
        id
    }

    pub fn get(&self, index: i32, snap_id: i32) -> i32 {
        let ref arr = self.arr[index as usize];
        let mut low;
        let mut low_val;
        let mut high = 0;
        if let Some(&(i, v)) = arr.first() {
            if snap_id < i {
                return 0;
            }
            if snap_id == i || arr.len() == 1 {
                return v;
            }
            low = 0;
            low_val = v;
        } else {
            return 0;
        }
        if let Some(&(i, v)) = arr.last() {
            if snap_id >= i {
                return v;
            }
            high = arr.len() - 1;
        }
        while low + 1 < high {
            let mid = (low + high) / 2;
            let (i, v) = arr[mid];
            if snap_id == i {
                return v;
            }
            if snap_id > i {
                low = mid;
                low_val = v;
            } else {
                high = mid;
            }
        }
        low_val
    }
}

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */
#[cfg(test)]
mod tests {
    use super::*;
    enum Op {
        Set(i32, i32),
        Snap(i32),
        Get(i32, i32, i32),
    }
    #[test]
    fn snap_array() {
        let tt = vec![
            (
                3,
                vec![Op::Set(0, 5), Op::Snap(0), Op::Set(0, 6), Op::Get(0, 0, 5)],
            ),
            (
                1,
                vec![
                    Op::Set(0, 4),
                    Op::Set(0, 16),
                    Op::Set(0, 13),
                    Op::Snap(0),
                    Op::Get(0, 0, 13),
                    Op::Snap(1),
                ],
            ),
        ];
        for (n, ops) in tt {
            let mut obj = SnapshotArray::new(n);
            for op in ops {
                match op {
                    Op::Set(i, v) => obj.set(i, v),
                    Op::Snap(i) => assert_eq!(obj.snap(), i),
                    Op::Get(idx, i, v) => assert_eq!(obj.get(idx, i), v),
                }
            }
        }
    }
}
