// https://leetcode.com/problems/minimum-possible-integer-after-at-most-k-adjacent-swaps-on-digits/
// 1505. Minimum Possible Integer After at Most K Adjacent Swaps On Digits
pub struct Solution;
use std::{cell::RefCell, rc::Rc};
#[derive(Clone, Debug, Default)]
struct Node {
    minv: char,
    mini: usize,
    cnt: i32,
    l: usize,
    r: usize,
}
impl Solution {
    fn combine(c: Rc<RefCell<Node>>, l: Rc<RefCell<Node>>, r: Rc<RefCell<Node>>) {
        let mut c = c.borrow_mut();
        let l = l.borrow();
        let r = r.borrow();
        if l.minv <= r.minv {
            c.minv = l.minv;
            c.mini = l.mini;
        } else {
            c.minv = r.minv;
            c.mini = r.mini;
        }
        c.cnt = l.cnt + r.cnt;
    }
    fn build(vs: &Vec<char>, vi: usize, l: usize, r: usize, vt: &Vec<Rc<RefCell<Node>>>) {
        vt[vi].borrow_mut().l = l;
        vt[vi].borrow_mut().r = r;
        if l == r {
            let mut node = vt[vi].borrow_mut();
            node.minv = vs[l];
            node.cnt = 1;
            node.mini = l;
        } else {
            let mid = (l + r) / 2;
            Self::build(vs, vi * 2 + 1, l, mid, vt);
            Self::build(vs, vi * 2 + 2, mid + 1, r, vt);
            Self::combine(vt[vi].clone(), vt[vi * 2 + 1].clone(), vt[vi * 2 + 2].clone());
        }
    }
    fn update(vt: &Vec<Rc<RefCell<Node>>>, vi: usize, pos: usize, c: char) {
        let l = vt[vi].borrow().l;
        let r = vt[vi].borrow().r;
        if l == pos && r == pos {
            vt[vi].borrow_mut().minv = c;
            vt[vi].borrow_mut().cnt = 0;
        } else {
            let mid = (l + r) / 2;
            if pos <= mid {
                Self::update(vt, vi * 2 + 1, pos, c);
            } else {
                Self::update(vt, vi * 2 + 2, pos, c);
            }
            Self::combine(vt[vi].clone(), vt[vi * 2 + 1].clone(), vt[vi * 2 + 2].clone());
        }
    }
    fn query(vt: &Vec<Rc<RefCell<Node>>>, vi: usize, l: usize, r: usize) -> (char, usize, i32) {
        let node = vt[vi].borrow();
        if node.l >= l && node.r <= r {
            (node.minv, node.mini, node.cnt)
        } else {
            let mid = (node.l + node.r) / 2;
            if r <= mid {
                Self::query(vt, vi * 2 + 1, l, r)
            } else if l > mid {
                Self::query(vt, vi * 2 + 2, l, r)
            } else {
                let (vl, il, cl) = Self::query(vt, vi * 2 + 1, l, mid);
                let (vr, ir, cr) = Self::query(vt, vi * 2 + 2, mid + 1, r);
                if vl <= vr {
                    (vl, il, cl + cr)
                } else {
                    (vr, ir, cl + cr)
                }
            }
        }
    }
    fn get_idx(vt: &Vec<Rc<RefCell<Node>>>, sum: i32, pos: usize) -> usize {
        let mut l = 0;
        let mut r = pos;
        if Self::query(vt, 0, 0, r).2 <= sum {
            return pos;
        }
        while l + 1 < r {
            let mid = (l + r) / 2;
            if Self::query(vt, 0, 0, mid).2 <= sum {
                l = mid;
            } else {
                r = mid;
            }
        }
        l
    }
    pub fn min_integer(num: String, k: i32) -> String {
        let mut vt = Vec::with_capacity(4 * num.len());
        for _ in 0..4 * num.len() {
            vt.push(Rc::new(RefCell::new(Node::default())));
        }
        let mut vs = num.chars().collect::<Vec<_>>();
        let n = vs.len();
        let mut k = k;
        if k as usize >= n * (n - 1) / 2 {
            vs.sort_unstable();
            return vs.into_iter().collect();
        }
        Self::build(&vs, 0, 0, vs.len() - 1, &mut vt);
        let mut vr = vec![];
        loop {
            let idx = Self::get_idx(&vt, k + 1, vs.len() - 1);
            let (minv, mini, _) = Self::query(&vt, 0, 0, idx);
            if minv == 'A' {
                break;
            }
            vr.push(minv);
            Self::update(&vt, 0, mini, 'A');
            k -= Self::query(&vt, 0, 0, mini).2;
        }
        vr.into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_integer() {
        assert_eq!(Solution::min_integer(String::from("4321"), 4), "1342");
        assert_eq!(Solution::min_integer(String::from("100"), 1), "010");
        assert_eq!(Solution::min_integer(String::from("36789"), 3), "36789");
    }
}
