// https://leetcode.com/problems/kth-ancestor-of-a-tree-node/
// 1483. Kth Ancestor of a Tree Node
pub struct TreeAncestor {
    ancestor: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TreeAncestor {
    pub fn new(_n: i32, parent: Vec<i32>) -> Self {
        let n = parent.len();
        let mut ancestor = vec![parent];
        loop {
            let mut has_ancestor = false;
            let last = ancestor.last().unwrap();
            let mut next = Vec::new();
            for i in 0..n {
                let pi = if last[i] == -1 {
                    -1
                } else {
                    last[last[i] as usize]
                };
                has_ancestor = has_ancestor || pi != -1;
                next.push(pi);
            }
            if has_ancestor {
                ancestor.push(next);
            } else {
                break;
            }
        }
        Self { ancestor }
    }

    pub fn get_kth_ancestor(&self, node: i32, k: i32) -> i32 {
        let mut bit = 1 << 16;
        let mut n = 16usize;
        let mut p = node;
        loop {
            if k & bit > 0 {
                if n >= self.ancestor.len() {
                    return -1;
                }
                p = self.ancestor[n][p as usize];
            }
            if p == -1 {
                return -1;
            }
            bit >>= 1;
            if bit == 0 {
                return p;
            }
            n -= 1;
        }
    }
}

/**
 * Your TreeAncestor object will be instantiated and called as such:
 * let obj = TreeAncestor::new(n, parent);
 * let ret_1: i32 = obj.get_kth_ancestor(node, k);
 */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tree_ancestor() {
        let ta = TreeAncestor::new(7, vec![-1, 0, 0, 1, 1, 2, 2]);
        assert_eq!(ta.get_kth_ancestor(3, 1), 1);
        assert_eq!(ta.get_kth_ancestor(5, 2), 0);
        assert_eq!(ta.get_kth_ancestor(6, 3), -1);
    }
}
