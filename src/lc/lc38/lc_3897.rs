// https://leetcode.com/problems/maximum-value-of-concatenated-binary-segments/
// 3897. Maximum Value of Concatenated Binary Segments
pub struct Solution;
impl Solution {
    pub fn max_value(nums1: Vec<i32>, nums0: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let n = nums1.len();
        let mut segs: Vec<(usize, usize)> = Vec::with_capacity(n);
        for i in 0..n {
            let a = nums1[i] as usize;
            let b = nums0[i] as usize;
            segs.push((a, b));
        }

        segs.sort_by(|&(a1, b1), &(a2, b2)| Self::cmp_segments_desc(a1, b1, a2, b2));

        let mut ans = 0i64;
        for (a, b) in segs {
            let len = a + b;
            let pow2_a = Self::pow2_mod(a as i64, MOD);
            let pow2_b = Self::pow2_mod(b as i64, MOD);
            let pow2_len = Self::pow2_mod(len as i64, MOD);

            let seg_val = ((pow2_a - 1 + MOD) % MOD) * pow2_b % MOD;
            ans = (ans * pow2_len + seg_val) % MOD;
        }

        ans as i32
    }

    fn pow2_mod(mut exp: i64, modu: i64) -> i64 {
        let mut base = 2i64;
        let mut res = 1i64;
        while exp > 0 {
            if exp & 1 == 1 {
                res = (res * base) % modu;
            }
            base = (base * base) % modu;
            exp >>= 1;
        }
        res
    }

    fn cmp_segments_desc(a1: usize, b1: usize, a2: usize, b2: usize) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        // Segment with b == 0 is all ones and should always be placed before any segment with zeros.
        if b1 == 0 && b2 == 0 {
            return Ordering::Equal;
        }
        if b1 == 0 {
            return Ordering::Less;
        }
        if b2 == 0 {
            return Ordering::Greater;
        }

        // For two segments with zeros, order by more leading ones first; if tied, fewer zeros first.
        match a2.cmp(&a1) {
            Ordering::Equal => b1.cmp(&b2),
            ord => ord,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_value() {
        assert_eq!(Solution::max_value(vec![1, 2], vec![1, 0]), 14);
        assert_eq!(Solution::max_value(vec![3, 1], vec![0, 3]), 120);
    }
}
