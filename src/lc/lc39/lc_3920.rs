// https://leetcode.com/problems/maximize-fixed-points-after-deletions/
// 3920. Maximize Fixed Points After Deletions
pub struct Solution;

struct FenwickMax {
    tree: Vec<i32>,
}

impl FenwickMax {
    fn new(n: usize) -> Self {
        Self { tree: vec![0; n + 1] }
    }

    fn update(&mut self, mut idx: usize, val: i32) {
        let n = self.tree.len() - 1;
        while idx <= n {
            self.tree[idx] = self.tree[idx].max(val);
            let lsb = idx & (!idx + 1);
            idx += lsb;
        }
    }

    fn query(&self, mut idx: usize) -> i32 {
        let mut best = 0;
        while idx > 0 {
            best = best.max(self.tree[idx]);
            let lsb = idx & (!idx + 1);
            idx -= lsb;
        }
        best
    }
}

impl Solution {
    pub fn max_fixed_points(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        // For an element at original index j with value v to be fixed eventually,
        // we need v <= j. Let u = j - v. Chosen fixed elements must have:
        // 1) strictly increasing v, and 2) nondecreasing u.
        // This becomes a grouped DP by v with prefix-max on u.
        let mut buckets: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        for (j, &v) in nums.iter().enumerate() {
            let vv = v as usize;
            if vv <= j {
                buckets[vv].push(j - vv);
            }
        }

        let mut bit = FenwickMax::new(n + 1);
        let mut ans = 0;

        for us in buckets.iter() {
            if us.is_empty() {
                continue;
            }

            let mut pending: Vec<(usize, i32)> = Vec::with_capacity(us.len());
            for &u in us {
                let best_prev = bit.query(u + 1);
                let cur = best_prev + 1;
                pending.push((u + 1, cur));
                ans = ans.max(cur);
            }

            // Apply updates after the whole value-group to keep v strictly increasing.
            for (idx, val) in pending {
                bit.update(idx, val);
            }
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_fixed_points() {
        assert_eq!(Solution::max_fixed_points(vec![0, 2, 1]), 2);
        assert_eq!(Solution::max_fixed_points(vec![3, 1, 2]), 2);
        assert_eq!(Solution::max_fixed_points(vec![1, 0, 1, 2]), 3);
    }
}
