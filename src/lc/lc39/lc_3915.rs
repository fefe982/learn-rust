// https://leetcode.com/problems/maximum-sum-of-alternating-subsequence-with-distance-at-least-k/
// 3915. Maximum Sum of Alternating Subsequence With Distance at Least K
pub struct Solution;

struct FenwickMax {
    tree: Vec<i64>,
}

impl FenwickMax {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![i64::MIN / 4; n + 1],
        }
    }

    fn update(&mut self, mut idx: usize, val: i64) {
        let n = self.tree.len() - 1;
        while idx <= n {
            self.tree[idx] = self.tree[idx].max(val);
            let lsb = idx & (!idx + 1);
            idx += lsb;
        }
    }

    fn query(&self, mut idx: usize) -> i64 {
        let mut best = i64::MIN / 4;
        while idx > 0 {
            best = best.max(self.tree[idx]);
            let lsb = idx & (!idx + 1);
            idx -= lsb;
        }
        best
    }
}

impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;

        let mut vals = nums.clone();
        vals.sort_unstable();
        vals.dedup();
        let m = vals.len();

        let mut pos = vec![0usize; n];
        for i in 0..n {
            // Coordinate-compress values so Fenwick trees can query by < / > relations.
            pos[i] = vals.binary_search(&nums[i]).unwrap() + 1;
        }

        let mut fenwick_down_prefix = FenwickMax::new(m);
        let mut fenwick_up_suffix_rev = FenwickMax::new(m);

        let mut dp_up = vec![0i64; n];
        let mut dp_down = vec![0i64; n];
        let mut ans = 0i64;

        for i in 0..n {
            if i >= k {
                let p = i - k;
                let pp = pos[p];
                let rev_pp = m - pp + 1;
                fenwick_down_prefix.update(pp, dp_down[p]);
                fenwick_up_suffix_rev.update(rev_pp, dp_up[p]);
            }

            let x = nums[i] as i64;
            let p = pos[i];
            let rev_p = m - p + 1;

            // Extend a sequence whose previous step was down (or a single-element sequence).
            let mut best_up = x;
            if p > 1 {
                let prev = fenwick_down_prefix.query(p - 1);
                if prev > i64::MIN / 8 {
                    best_up = best_up.max(x + prev);
                }
            }

            // Extend a sequence whose previous step was up (or a single-element sequence).
            let mut best_down = x;
            if rev_p > 1 {
                let prev = fenwick_up_suffix_rev.query(rev_p - 1);
                if prev > i64::MIN / 8 {
                    best_down = best_down.max(x + prev);
                }
            }

            dp_up[i] = best_up;
            dp_down[i] = best_down;
            ans = ans.max(best_up).max(best_down);
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_alternating_sum() {
        assert_eq!(Solution::max_alternating_sum(vec![5, 4, 2], 2), 7);
        assert_eq!(Solution::max_alternating_sum(vec![3, 5, 4, 2, 4], 1), 14);
        assert_eq!(Solution::max_alternating_sum(vec![5], 1), 5);
    }
}
