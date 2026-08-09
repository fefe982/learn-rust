// https://leetcode.com/problems/peaks-in-array-ii/
// 4017. Peaks in Array II
pub struct Solution;
#[derive(Debug, Copy, Clone, Default)]
struct Val {
    cnt: i64,
    pre: i32,
    suf: i32,
    len: i32,
    has: bool,
}

impl Solution {
    fn create_tree(nums: &Vec<i32>) -> Vec<Val> {
        let mut len = 1;
        while len < nums.len() {
            len <<= 1;
        }
        let mut tree = vec![Val::default(); len << 1];
        Self::build_tree(&mut tree, nums, 0, 0, nums.len());
        tree
    }
    fn build_tree(tree: &mut Vec<Val>, nums: &Vec<i32>, i: usize, l: usize, r: usize) {
        if l + 1 == r {
            let peak = l > 0 && l < nums.len() - 1 && nums[l - 1] < nums[l] && nums[l] > nums[l + 1];
            tree[i].cnt = 0;
            tree[i].pre = 1;
            tree[i].suf = 1;
            tree[i].len = 1;
            tree[i].has = peak;
            return;
        }
        let mid = (l + r) / 2;
        Self::build_tree(tree, nums, i * 2 + 1, l, mid);
        Self::build_tree(tree, nums, i * 2 + 2, mid, r);
        Self::maintain(tree, i)
    }
    fn maintain(tree: &mut Vec<Val>, i: usize) {
        let val = Self::merge(&tree[i * 2 + 1], &tree[i * 2 + 2]);
        tree[i] = val;
    }
    fn merge(left: &Val, right: &Val) -> Val {
        Val {
            cnt: left.cnt + right.cnt + left.len as i64 * right.len as i64 - left.suf as i64 * right.pre as i64,
            pre: if left.has { left.pre } else { left.len + right.pre },
            suf: if right.has { right.suf } else { right.len + left.suf },
            len: left.len + right.len,
            has: left.has || right.has,
        }
    }
    fn update(tree: &mut Vec<Val>, i: usize, l: usize, r: usize, idx: usize) {
        if l + 1 == r {
            tree[i].has = !tree[i].has;
            return;
        }
        let mid = (l + r) / 2;
        if idx < mid {
            Self::update(tree, i * 2 + 1, l, mid, idx);
        } else {
            Self::update(tree, i * 2 + 2, mid, r, idx);
        }
        Self::maintain(tree, i);
    }
    fn query(tree: &Vec<Val>, i: usize, l: usize, r: usize, ql: usize, qr: usize) -> Val {
        if ql <= l && r <= qr {
            return tree[i];
        }
        let mid = (l + r) / 2;
        if qr <= mid {
            return Self::query(tree, i * 2 + 1, l, mid, ql, qr);
        }
        if ql >= mid {
            return Self::query(tree, i * 2 + 2, mid, r, ql, qr);
        }
        Self::merge(
            &Self::query(tree, i * 2 + 1, l, mid, ql, qr),
            &Self::query(tree, i * 2 + 2, mid, r, ql, qr),
        )
    }
    pub fn count_of_peaks(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let n = nums.len();
        let mut tree = Self::create_tree(&nums);
        let mut ans = vec![];
        let mut nums = nums;
        for query in queries {
            let op = query[0];
            if op == 1 {
                ans.push(Self::query(&tree, 0, 0, n, query[1] as usize, query[2] as usize + 1).cnt);
                continue;
            }
            let i = query[1] as usize;
            let v = query[2];
            if i > 1 {
                let ohas = nums[i - 1] > nums[i - 2] && nums[i - 1] > nums[i];
                let nhas = nums[i - 1] > nums[i - 2] && nums[i - 1] > v;
                if ohas != nhas {
                    Self::update(&mut tree, 0, 0, n, i - 1);
                }
            }
            if i > 0 && i < n - 1 {
                let ohas = nums[i] > nums[i - 1] && nums[i] > nums[i + 1];
                let nhas = v > nums[i - 1] && v > nums[i + 1];
                if ohas != nhas {
                    Self::update(&mut tree, 0, 0, n, i);
                }
            }
            if i < n - 2 {
                let ohas = nums[i + 1] > nums[i] && nums[i + 1] > nums[i + 2];
                let nhas = nums[i + 1] > v && nums[i + 1] > nums[i + 2];
                if ohas != nhas {
                    Self::update(&mut tree, 0, 0, n, i + 1);
                }
            }
            nums[i] = v;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_of_peaks() {
        assert_eq!(
            Solution::count_of_peaks(vec![1, 3, 2, 4], vec_vec![[1, 0, 3], [2, 1, 1], [1, 0, 3]]),
            [2, 0]
        );
        assert_eq!(
            Solution::count_of_peaks(vec![9, 8, 9, 8], vec_vec![[1, 1, 3], [2, 2, 1], [1, 0, 2]]),
            [1, 0]
        );
        assert_eq!(
            Solution::count_of_peaks(vec![3, 6, 2, 7, 1], vec_vec![[1, 1, 3], [2, 3, 0], [1, 0, 4]]),
            [0, 3]
        );
    }
}
