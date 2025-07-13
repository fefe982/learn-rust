// https://leetcode.com/problems/design-skiplist/
// 1206. Design Skiplist
use rand;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;
struct Node {
    v: i32,
    cnt: i32,
    next: Vec<Option<Rc<RefCell<Node>>>>,
}
pub struct Skiplist {
    head: Option<Rc<RefCell<Node>>>,
    cnt: usize,
    rnd: rand::rngs::ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Skiplist {
    pub fn new() -> Self {
        Self {
            head: Some(Rc::new(RefCell::new(Node {
                v: i32::MIN,
                cnt: 1,
                next: vec![None],
            }))),
            cnt: 0,
            rnd: rand::thread_rng(),
        }
    }

    fn search_internal(&self, target: i32) -> (Vec<Option<Rc<RefCell<Node>>>>, Option<Rc<RefCell<Node>>>) {
        let mut curr = self.head.clone();
        let mut res = vec![];
        let mut level = curr.clone().unwrap().borrow().next.len() - 1;
        let mut prev = None;
        loop {
            while let Some(node) = curr {
                let v = node.borrow().v;
                if v == target {
                    res.push(prev);
                    return (res, Some(node));
                }
                if v < target {
                    prev = Some(node.clone());
                    curr = node.borrow().next[level].clone();
                } else {
                    break;
                }
            }
            res.push(prev.clone());
            if level == 0 {
                return (res, None);
            }
            level -= 1;
            curr = prev.clone().unwrap().borrow().next[level].clone();
        }
    }

    pub fn search(&self, target: i32) -> bool {
        self.search_internal(target).1.is_some()
    }

    pub fn add(&mut self, num: i32) {
        let (prev, found) = self.search_internal(num);
        if let Some(node) = found {
            node.borrow_mut().cnt += 1;
            return;
        }
        self.cnt += 1;
        let max_level = (usize::BITS - self.cnt.leading_zeros()) as usize;
        let level = self.rnd.gen::<usize>() % max_level;
        let node = Rc::new(RefCell::new(Node {
            v: num,
            cnt: 1,
            next: vec![None; level as usize + 1],
        }));
        for (lvl, p) in prev.iter().rev().enumerate() {
            if lvl > level {
                break;
            }
            let pnode = p.clone().unwrap();
            node.borrow_mut().next[lvl] = pnode.borrow().next[lvl].clone();
            pnode.borrow_mut().next[lvl] = Some(node.clone());
        }
        while self.head.as_ref().unwrap().borrow().next.len() <= level {
            self.head.as_ref().unwrap().borrow_mut().next.push(Some(node.clone()));
        }
    }

    pub fn erase(&mut self, num: i32) -> bool {
        let (prev, found) = self.search_internal(num);
        if let Some(node) = found {
            node.borrow_mut().cnt -= 1;
            if node.borrow().cnt > 0 {
                return true;
            }
        } else {
            return false;
        }
        self.cnt -= 1;
        let mut level = prev[0].clone().unwrap().borrow().next.len() - prev.len();
        let mut prev = prev.last().unwrap().clone().unwrap();
        let node = prev.borrow_mut().next[level].take().unwrap();
        loop {
            prev.borrow_mut().next[level] = node.borrow_mut().next[level].take();
            if level == 0 {
                break;
            }
            level -= 1;
            loop {
                let pnode = prev.borrow().next[level].clone().unwrap();
                if pnode.borrow().v != num {
                    prev = pnode
                } else {
                    break;
                }
            }
        }
        true
    }
}

/**
 * Your Skiplist object will be instantiated and called as such:
 * let obj = Skiplist::new();
 * let ret_1: bool = obj.search(target);
 * obj.add(num);
 * let ret_3: bool = obj.erase(num);
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[derive(Copy, Clone)]
    enum Op {
        Add(i32),
        Search(i32, bool),
        Erase(i32, bool),
    }
    #[test]
    fn test() {
        let ops = vec_vec![
            [
                Op::Add(16),
                Op::Add(5),
                Op::Add(14),
                Op::Add(13),
                Op::Add(0),
                Op::Add(3),
                Op::Add(12),
                Op::Add(9),
                Op::Add(12),
                Op::Erase(3, true),
                Op::Search(6, false),
                Op::Add(7),
                Op::Erase(0, true),
            ],
            [
                Op::Add(0),
                Op::Add(5),
                Op::Add(2),
                Op::Add(1),
                Op::Search(0, true),
                Op::Erase(5, true),
                Op::Search(2, true),
                Op::Search(3, false),
                Op::Search(2, true)
            ],
            [
                Op::Add(1),
                Op::Add(2),
                Op::Add(3),
                Op::Search(0, false),
                Op::Add(4),
                Op::Search(1, true),
                Op::Erase(0, false),
                Op::Erase(1, true),
            ]
        ];
        for v in ops {
            let mut obj = Skiplist::new();
            for op in v {
                match op {
                    Op::Add(num) => obj.add(num),
                    Op::Search(num, expect) => assert!(obj.search(num) == expect),
                    Op::Erase(num, expect) => assert!(obj.erase(num) == expect),
                }
            }
        }
    }
}
