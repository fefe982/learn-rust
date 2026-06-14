// https://leetcode.com/problems/maximum-subarray-sum-after-at-most-k-swaps/
// 3962. Maximum Subarray Sum After At Most K Swaps
pub struct Solution;
struct Fenwick<'a> {
    n: usize,
    w: usize,
    sorted: &'a [i32],
    cnt: Vec<i32>,
    sum: Vec<i64>,
}

impl Clone for Fenwick<'_> {
    fn clone(&self) -> Self {
        Fenwick {
            n: self.n,
            w: self.w,
            sorted: self.sorted,
            cnt: self.cnt.clone(),
            sum: self.sum.clone(),
        }
    }
}

impl<'a> Fenwick<'a> {
    fn new(n: usize, sorted: &'a [i32], width: usize) -> Self {
        Fenwick {
            n,
            w: width,
            sorted,
            cnt: vec![0; n],
            sum: vec![0; n],
        }
    }

    fn update(&mut self, mut i: usize, delta_cnt: i32, delta_sum: i64) {
        while i < self.n {
            self.cnt[i] += delta_cnt;
            self.sum[i] += delta_sum;
            i += i & (!i + 1);
        }
    }

    fn query_sum(&self, mut k: i32) -> i64 {
        let mut i = 0;
        let mut bit = 1 << (self.w - 1);
        let mut res = 0;
        while bit > 0 {
            let next_i = i | bit;
            if next_i < self.n && self.cnt[next_i] < k {
                k -= self.cnt[next_i];
                res += self.sum[next_i];
                i = next_i;
            }
            bit >>= 1;
        }
        res + (k as i64) * (self.sorted[i] as i64)
    }
}

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut all_pos_sum = 0;
        let mut all_pos_cnt = 0;
        for &x in &nums {
            if x > 0 {
                all_pos_sum += x as i64;
                all_pos_cnt += 1;
            }
        }

        if all_pos_cnt == 0 {
            return nums.into_iter().max().unwrap() as i64;
        }
        let mut cnt = 0;
        let k = k;
        for (i, &x) in nums.iter().enumerate() {
            if x > 0 {
                cnt += 1;
            }
            if i + 1 < all_pos_cnt as usize {
                continue;
            }
            if cnt + k >= all_pos_cnt {
                return all_pos_sum;
            }
            if nums[i + 1 - all_pos_cnt as usize] > 0 {
                cnt -= 1;
            }
        }

        let mut sorted = nums.clone();
        sorted.sort_unstable();
        let mut m = 0;
        for i in 1..sorted.len() {
            if sorted[i] != sorted[m] {
                m += 1;
                sorted[m] = sorted[i];
            }
        }
        sorted.truncate(m + 1);
        let width = (m + 1).next_power_of_two().trailing_zeros() as usize;
        let mut rank = vec![0; nums.len()];
        let mut all_pos_tree = Fenwick::new(m + 1, &sorted, width);
        for (i, &x) in nums.iter().enumerate() {
            let r = sorted.binary_search(&x).unwrap() + 1;
            rank[i] = r;
            if x > 0 {
                all_pos_tree.update(r, 1, x as i64);
            }
        }

        let mut ans = i64::MIN;
        let n = nums.len();
        // Try each starting position
        for left in 0..n {
            let mut neg_tree = Fenwick::new(m + 1, &sorted, width);
            let mut pos_tree = all_pos_tree.clone();
            let mut pos_sum = all_pos_sum;
            let mut pos_cnt = all_pos_cnt;
            let mut sub_sum = 0;
            let mut neg_cnt = 0;
            for right in left..n {
                let x = nums[right];
                let rx = rank[right];
                sub_sum += x as i64;
                if x > 0 {
                    pos_tree.update(rx, -1, -(x as i64));
                    pos_sum -= x as i64;
                    pos_cnt -= 1;
                } else if x < 0 {
                    neg_tree.update(rx, 1, x as i64);
                    neg_cnt += 1;
                }
                let mut delta = 0;
                let swap = neg_cnt.min(pos_cnt).min(k) as i32;
                if swap > 0 {
                    let neg = neg_tree.query_sum(swap);
                    let pos = pos_sum - pos_tree.query_sum(pos_cnt - swap);
                    delta = pos - neg;
                }
                ans = ans.max(sub_sum + delta);
            }
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_sum() {
        assert_eq!(Solution::max_sum(vec![21, -4, 46], 0), 63);
        assert_eq!(Solution::max_sum(vec![1, -1, 0, 2], 1), 3);
        assert_eq!(Solution::max_sum(vec![4, 3, 2, 4], 2), 13);
        assert_eq!(Solution::max_sum(vec![-1, -2], 0), -1);
    }
}
