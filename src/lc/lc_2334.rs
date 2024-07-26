// https://leetcode.com/problems/subarray-with-elements-greater-than-varying-threshold/
// 2334. Subarray With Elements Greater Than Varying Threshold
pub struct Solution;
impl Solution {
    fn set(v: &mut Vec<(i32, usize)>, idx: usize, l: usize, r: usize, pos: usize, val: i32) {
        if l + 1 == r {
            v[idx] = (val, pos);
            return;
        }
        let mid = (l + r) / 2;
        if pos < mid {
            Self::set(v, (idx << 1) + 1, l, mid, pos, val);
            if v[idx].0 > v[(idx << 1) + 1].0 {
                v[idx] = v[(idx << 1) + 1];
            }
        } else {
            Self::set(v, (idx << 1) + 2, mid, r, pos, val);
            if v[idx].0 > v[(idx << 1) + 2].0 {
                v[idx] = v[(idx << 1) + 2];
            }
        }
    }
    fn get(v: &Vec<(i32, usize)>, idx: usize, l: usize, r: usize, ql: usize, qr: usize) -> (i32, usize) {
        if ql <= l && r <= qr {
            return v[idx];
        }
        if qr <= l || r <= ql {
            return (i32::MAX, usize::MAX);
        }
        let mid = (l + r) / 2;
        let le = Self::get(v, (idx << 1) + 1, l, mid, ql, qr);
        let re = Self::get(v, (idx << 1) + 2, mid, r, ql, qr);
        if le.0 < re.0 {
            le
        } else {
            re
        }
    }
    fn check(v: &Vec<(i32, usize)>, threshold: i32, l: usize, r: usize, sz: usize) -> i32 {
        if l == r {
            return -1;
        }
        let (min, imin) = Self::get(v, 0, 0, sz, l, r);
        if min > threshold / (r - l) as i32 {
            return (r - l) as i32;
        }
        let lc = Self::check(v, threshold, l, imin, sz);
        if lc > 0 {
            return lc;
        }
        return Self::check(v, threshold, imin + 1, r, sz);
    }
    pub fn valid_subarray_size(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut sz = 1;
        let n = nums.len();
        while sz < n {
            sz <<= 1;
        }
        let mut v = vec![(i32::MAX, usize::MAX); (sz << 1) - 1];
        for (idx, n) in nums.into_iter().enumerate() {
            Self::set(&mut v, 0, 0, sz, idx, n);
        }
        Self::check(&v, threshold, 0, n, sz)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_subarray_size() {
        assert_eq!(Solution::valid_subarray_size(vec![1, 3, 4, 3, 1], 6), 3);
        assert_eq!(Solution::valid_subarray_size(vec![6, 5, 6, 5, 8], 7), 5);
    }
}
