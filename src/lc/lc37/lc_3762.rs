// https://leetcode.cn/problems/minimum-operations-to-equalize-subarrays/
// 3762. Minimum Operations to Equalize Subarrays
pub struct Solution;
use std::{cell::RefCell, ops::Deref, rc::Rc};
#[derive(Debug, Clone)]
struct SegmentTree {
    cnt: i64,
    sum: i64,
    l: usize,
    r: usize,
    left: Option<Rc<RefCell<SegmentTree>>>,
    right: Option<Rc<RefCell<SegmentTree>>>,
}
impl SegmentTree {
    fn new(l: usize, r: usize) -> Self {
        let mut left = None;
        let mut right = None;
        if l + 1 < r {
            let mid = (l + r) / 2;
            left = Some(Rc::new(RefCell::new(SegmentTree::new(l, mid))));
            right = Some(Rc::new(RefCell::new(SegmentTree::new(mid, r))));
        }
        Self {
            cnt: 0,
            sum: 0,
            l,
            r,
            left,
            right,
        }
    }
    fn add(&self, idx: usize, val: i64) -> Self {
        let mut self_ = self.clone();
        self_.cnt += 1;
        self_.sum += val;
        if self_.l + 1 == self_.r {
            return self_;
        }
        let mid = (self.l + self.r) / 2;
        if idx < mid {
            self_.left = Some(Rc::new(RefCell::new(self_.left.take().unwrap().borrow().add(idx, val))));
        } else {
            self_.right = Some(Rc::new(RefCell::new(
                self_.right.take().unwrap().borrow().add(idx, val),
            )));
        }
        return self_;
    }
    fn kth(&self, sub: &SegmentTree, k: i64) -> (usize, i64, i64) {
        if self.l + 1 == self.r {
            return (self.l, 0, 0);
        }
        let l = self.left.as_ref().unwrap().borrow().cnt - sub.left.as_ref().unwrap().borrow().cnt;
        if l <= k {
            let (mid, cnt, sum) = self
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .kth(sub.right.as_ref().unwrap().borrow().deref(), k - l);
            (
                mid,
                cnt + self.left.as_ref().unwrap().borrow().cnt - sub.left.as_ref().unwrap().borrow().cnt,
                sum + self.left.as_ref().unwrap().borrow().sum - sub.left.as_ref().unwrap().borrow().sum,
            )
        } else {
            self.left
                .as_ref()
                .unwrap()
                .borrow()
                .kth(sub.left.as_ref().unwrap().borrow().deref(), k)
        }
    }
}
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let n = nums.len();
        let mut mod_left = vec![0; n];
        for i in 1..n {
            mod_left[i] = if nums[i] % k == nums[i - 1] % k {
                mod_left[i - 1]
            } else {
                i
            };
        }
        let mut s = std::collections::BTreeMap::new();
        for i in 0..n {
            s.insert(nums[i], 0);
        }
        let mut snums = Vec::with_capacity(s.len());
        for (i, (k, v)) in s.iter_mut().enumerate() {
            *v = i;
            snums.push(*k as i64);
        }
        let mut roots = Vec::with_capacity(n + 1);
        roots.push(SegmentTree::new(0, n));
        for i in 0..n {
            roots.push(roots[i].add(s[&nums[i]], nums[i] as i64));
        }
        let mut ans = Vec::with_capacity(queries.len());
        for query in queries {
            let (l, r) = (query[0] as usize, query[1] as usize);
            if mod_left[r] > l {
                ans.push(-1);
                continue;
            }
            let (mid, lcnt, lsum) = roots[r + 1].kth(&roots[l], ((r - l) / 2) as i64);
            let sum = roots[r + 1].sum - roots[l].sum;
            let median = snums[mid];
            let s = median * lcnt - lsum + (sum - lsum - median * (roots[r + 1].cnt - roots[l].cnt - lcnt));
            ans.push(s / k as i64);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_operations() {
        assert_eq!(
            Solution::min_operations(vec![1, 4, 7], 3, vec_vec![[0, 1], [0, 2]]),
            vec![1, 2]
        );
        assert_eq!(
            Solution::min_operations(vec![1, 2, 4], 2, vec_vec![[0, 2], [0, 0], [1, 2]]),
            vec![-1, 0, 1]
        );
    }
}
