// https://leetcode.com/problems/longest-balanced-subarray-ii/
// 3721. Longest Balanced Subarray II
pub struct Solution;
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct SegTreeNode {
    max: i32,
    min: i32,
    tag: i32,
}
impl SegTreeNode {
    fn new() -> Self {
        Self { max: 0, min: 0, tag: 0 }
    }
    fn apply(&mut self, val: i32) {
        self.max += val;
        self.min += val;
        self.tag += val;
    }
}
#[derive(Debug)]
struct SegTree {
    tree: Vec<SegTreeNode>,
    n: usize,
}
impl SegTree {
    fn new(n: usize) -> Self {
        let mut i = 1;
        while i < n {
            i <<= 1;
        }
        let tree = vec![SegTreeNode::new(); i << 2];
        Self { tree, n: i }
    }
    fn spread(&mut self, idx: usize) {
        let tag = self.tree[idx].tag;
        if tag == 0 {
            return;
        }
        self.tree[2 * idx + 1].apply(tag);
        self.tree[2 * idx + 2].apply(tag);
        self.tree[idx].tag = 0;
    }
    fn update_i(&mut self, idx: usize, rl: usize, rr: usize, l: usize, r: usize, val: i32) {
        if l <= rl && rr <= r {
            if rl + 1 < rr {
                self.tree[idx].tag += val;
            }
            self.tree[idx].max += val;
            self.tree[idx].min += val;
            return;
        }
        self.spread(idx);
        let mid = (rl + rr) >> 1;
        if l < mid {
            self.update_i(2 * idx + 1, rl, mid, l, r, val);
        }
        if r > mid {
            self.update_i(2 * idx + 2, mid, rr, l, r, val);
        }
        self.tree[idx].max = self.tree[2 * idx + 1].max.max(self.tree[2 * idx + 2].max);
        self.tree[idx].min = self.tree[2 * idx + 1].min.min(self.tree[2 * idx + 2].min)
    }
    fn update(&mut self, l: usize, r: usize, val: i32) {
        self.update_i(0, 0, self.n, l, r, val);
    }
    fn locate_i(&mut self, idx: usize, rl: usize, rr: usize, l: usize, r: usize, val: i32) -> usize {
        if rr <= l || r <= rl {
            return usize::MAX;
        }
        if l <= rl && rr <= r {
            if val > self.tree[idx].max || val < self.tree[idx].min {
                return usize::MAX;
            }
            if rl + 1 == rr {
                return rl;
            }
        }
        self.spread(idx);
        let mid = (rl + rr) >> 1;
        let mut ans = self.locate_i(2 * idx + 1, rl, mid, l, r, val);
        if ans == usize::MAX {
            ans = self.locate_i(2 * idx + 2, mid, rr, l, r, val);
        }
        ans
    }
    fn locate(&mut self, r: usize, val: i32) -> usize {
        if r == 0 {
            return usize::MAX;
        }
        self.locate_i(0, 0, self.n, 0, r, val)
    }
}
impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let mut seg = SegTree::new(nums.len() + 1);
        let mut last_pos = std::collections::HashMap::<i32, usize>::new();
        let mut ans = 0;
        let mut val = 0;
        for (i, num) in nums.into_iter().enumerate() {
            let v = num % 2 * 2 - 1;
            if let Some(last) = last_pos.insert(num, i) {
                seg.update(last + 1, i + 1, -v);
            } else {
                val += v;
            }
            let l = seg.locate(i + 1, val);
            if l != usize::MAX {
                ans = ans.max(i - l + 1);
            };
            seg.update(i + 1, i + 2, val);
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_balanced() {
        assert_eq!(Solution::longest_balanced(vec![9, 20, 5, 11, 20, 20]), 3);
        assert_eq!(Solution::longest_balanced(vec![2, 5, 4, 3]), 4);
        assert_eq!(Solution::longest_balanced(vec![3, 2, 2, 5, 4]), 5);
        assert_eq!(Solution::longest_balanced(vec![1, 2, 3, 2]), 3);
    }
}
