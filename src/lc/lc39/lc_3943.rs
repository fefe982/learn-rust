// https://leetcode.com/problems/number-of-pairs-after-increment/
// 3943. Number of Pairs After Increment
pub struct Solution;
use std::collections::HashMap;
struct BlockArray {
    n: usize,
    block_size: usize,
    vals: Vec<i64>,
    lazy: Vec<i64>,
    sorted: Vec<Vec<i64>>,
}

impl BlockArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let block_size = ((n as f64).sqrt() as usize).max(1);
        let block_cnt = n.div_ceil(block_size);

        let mut this = Self {
            n,
            block_size,
            vals: nums.into_iter().map(|x| x as i64).collect(),
            lazy: vec![0; block_cnt],
            sorted: vec![Vec::new(); block_cnt],
        };

        for b in 0..block_cnt {
            this.rebuild(b);
        }

        this
    }

    fn block_range(&self, b: usize) -> (usize, usize) {
        let l = b * self.block_size;
        let r = ((b + 1) * self.block_size).min(self.n) - 1;
        (l, r)
    }

    fn rebuild(&mut self, b: usize) {
        let (l, r) = self.block_range(b);
        self.sorted[b] = self.vals[l..=r].to_vec();
        self.sorted[b].sort_unstable();
    }

    fn range_add(&mut self, l: usize, r: usize, delta: i64) {
        let bl = l / self.block_size;
        let br = r / self.block_size;

        if bl == br {
            for i in l..=r {
                self.vals[i] += delta;
            }
            self.rebuild(bl);
            return;
        }

        let (_, bl_r) = self.block_range(bl);
        for i in l..=bl_r {
            self.vals[i] += delta;
        }
        self.rebuild(bl);

        let (br_l, _) = self.block_range(br);
        for i in br_l..=r {
            self.vals[i] += delta;
        }
        self.rebuild(br);

        for b in (bl + 1)..br {
            self.lazy[b] += delta;
        }
    }

    fn lower_bound(arr: &[i64], target: i64) -> usize {
        let mut l = 0;
        let mut r = arr.len();
        while l < r {
            let m = l + (r - l) / 2;
            if arr[m] < target {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l
    }

    fn upper_bound(arr: &[i64], target: i64) -> usize {
        let mut l = 0;
        let mut r = arr.len();
        while l < r {
            let m = l + (r - l) / 2;
            if arr[m] <= target {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l
    }

    fn count_equal(&self, target: i64) -> i64 {
        let mut res = 0_i64;
        for b in 0..self.sorted.len() {
            let want = target - self.lazy[b];
            let lo = Self::lower_bound(&self.sorted[b], want);
            let hi = Self::upper_bound(&self.sorted[b], want);
            res += (hi - lo) as i64;
        }
        res
    }
}

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums1_freq = HashMap::<i64, i64>::new();
        for x in nums1 {
            *nums1_freq.entry(x as i64).or_insert(0) += 1;
        }
        let nums1_vals: Vec<(i64, i64)> = nums1_freq.into_iter().collect();

        let mut arr = BlockArray::new(nums2);
        let mut ans = Vec::new();

        for q in queries {
            if q[0] == 1 {
                let x = q[1] as usize;
                let y = q[2] as usize;
                let val = q[3] as i64;
                arr.range_add(x, y, val);
            } else {
                let tot = q[1] as i64;
                let mut pairs = 0_i64;
                for &(a, cnt1) in &nums1_vals {
                    pairs += cnt1 * arr.count_equal(tot - a);
                }
                ans.push(pairs as i32);
            }
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_number_of_pairs() {
        assert_eq!(
            Solution::number_of_pairs(vec![1, 2], vec![3, 4], vec_vec![[2, 5], [1, 0, 0, 2], [2, 5]]),
            vec![2, 1]
        );
        assert_eq!(
            Solution::number_of_pairs(vec![1, 1], vec![2, 2, 3], vec_vec![[2, 4], [1, 0, 1, 1], [2, 4]]),
            vec![2, 6]
        );
        assert_eq!(
            Solution::number_of_pairs(vec![2, 5, 8, 4], vec![1, 3, 8], vec_vec![[2, 9], [1, 1, 2, 1], [2, 10]]),
            vec![1, 0]
        );
    }
}
