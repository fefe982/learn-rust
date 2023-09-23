// https://leetcode.com/problems/operations-on-tree/
// 1993. Operations on Tree
struct LockingTree {
    p: Vec<i32>,
    d: Vec<Vec<i32>>,
    l: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LockingTree {
    fn new(parent: Vec<i32>) -> Self {
        let mut d = vec![vec![]; parent.len()];
        for (i, &p) in parent.iter().enumerate() {
            if p != -1 {
                d[p as usize].push(i as i32);
            }
        }
        let l = vec![0; parent.len()];
        Self { p: parent, d, l }
    }

    fn lock(&mut self, num: i32, user: i32) -> bool {
        if self.l[num as usize] == 0 {
            self.l[num as usize] = user;
            true
        } else {
            false
        }
    }

    fn unlock(&mut self, num: i32, user: i32) -> bool {
        if self.l[num as usize] == user {
            self.l[num as usize] = 0;
            true
        } else {
            false
        }
    }

    fn upgrade(&mut self, num: i32, user: i32) -> bool {
        if self.l[num as usize] != 0 {
            return false;
        }
        let mut n = num as usize;
        while self.p[n] != -1 {
            n = self.p[n] as usize;
            if self.l[n] != 0 {
                return false;
            }
        }
        let mut nd = 0;
        let mut q = vec![num as usize];
        while let Some(c) = q.pop() {
            if self.l[c] != 0 {
                self.l[c] = 0;
                nd += 1;
            }
            for &d in self.d[c].iter() {
                q.push(d as usize);
            }
        }
        if nd == 0 {
            return false;
        }
        self.l[num as usize] = user;
        true
    }
}

/**
 * Your LockingTree object will be instantiated and called as such:
 * let obj = LockingTree::new(parent);
 * let ret_1: bool = obj.lock(num, user);
 * let ret_2: bool = obj.unlock(num, user);
 * let ret_3: bool = obj.upgrade(num, user);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut obj = LockingTree::new(vec![-1, 0, 0, 1, 1, 2, 2]);
        assert_eq!(obj.lock(2, 2), true);
        assert_eq!(obj.unlock(2, 3), false);
        assert_eq!(obj.unlock(2, 2), true);
        assert_eq!(obj.lock(4, 5), true);
        assert_eq!(obj.upgrade(0, 1), true);
        assert_eq!(obj.lock(0, 1), false);
    }
}
