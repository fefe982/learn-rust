// https://leetcode.cn/problems/exam-room/
// 855. Exam Room
#[derive(PartialEq, Eq)]
struct Rng {
    left: i32,
    right: i32,
    n: i32,
}
impl Rng {
    fn new(left: i32, right: i32, n: i32) -> Self {
        Self { left, right, n }
    }
    fn len(&self) -> i32 {
        match (self.left, self.right) {
            (-1, i32::MAX) => self.n,
            (-1, r) => r,
            (l, i32::MAX) => self.n - 1 - l,
            (l, r) => (r - l) / 2,
        }
    }
    fn mid(&self) -> i32 {
        match (self.left, self.right) {
            (-1, _) => 0,
            (_, i32::MAX) => self.n - 1,
            (l, r) => l + (r - l) / 2,
        }
    }
}
impl std::cmp::PartialOrd for Rng {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.len().cmp(&other.len()) {
            std::cmp::Ordering::Equal => Some(other.left.cmp(&self.left)),
            o => Some(o),
        }
    }
}
impl std::cmp::Ord for Rng {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}
struct ExamRoom {
    rngs: std::collections::BinaryHeap<Rng>,
    n: i32,
    seats: std::collections::BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ExamRoom {
    fn new(n: i32) -> Self {
        Self {
            rngs: std::collections::BinaryHeap::from([Rng::new(-1, i32::MAX, n)]),
            n,
            seats: std::collections::BTreeMap::from([(-1, i32::MAX)]),
        }
    }

    fn seat(&mut self) -> i32 {
        while let Some(rng) = self.rngs.pop() {
            if let Some(&r) = self.seats.get(&rng.left) {
                if r != rng.right {
                    continue;
                } else {
                    let mid = rng.mid();
                    self.seats.insert(rng.left, mid);
                    self.seats.insert(mid, rng.right);
                    self.rngs.push(Rng::new(rng.left, mid, self.n));
                    self.rngs.push(Rng::new(mid, rng.right, self.n));
                    return mid;
                }
            }
        }
        -1
    }

    fn leave(&mut self, p: i32) {
        let r = self.seats.remove(&p).unwrap();
        let l = *self.seats.range(..p).rev().next().unwrap().0;
        self.seats.insert(l, r);
        self.rngs.push(Rng::new(l, r, self.n));
    }
}

/**
 * Your ExamRoom object will be instantiated and called as such:
 * let obj = ExamRoom::new(n);
 * let ret_1: i32 = obj.seat();
 * obj.leave(p);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_855() {
        let mut obj = ExamRoom::new(10);
        assert_eq!(obj.seat(), 0);
        assert_eq!(obj.seat(), 9);
        assert_eq!(obj.seat(), 4);
        assert_eq!(obj.seat(), 2);
        obj.leave(4);
        assert_eq!(obj.seat(), 5);
    }
}
