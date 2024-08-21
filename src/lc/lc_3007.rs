// https://leetcode.com/problems/maximum-number-that-sum-of-the-prices-is-less-than-or-equal-to-k/
// 3007. Maximum Number That Sum of the Prices Is Less Than or Equal to K
pub struct Solution;
impl Solution {
    fn cnt_n(bit: i32, pre: i32, x: i32) -> i128 {
        (1 << bit) * (bit / x) as i128 / 2 + pre as i128 * (1 << bit)
    }
    fn count(n: i64, x: i32) -> i128 {
        let mut total = 0;
        let mut n = n;
        let mut pre = 0;
        while n > 0 {
            let nbit = 64 - n.leading_zeros() as i32;
            total += Solution::cnt_n(nbit - 1, pre, x);
            if nbit % x == 0 {
                total += 1;
                pre += 1;
            }
            n &= !(1 << (nbit - 1));
        }
        total
    }
    pub fn find_maximum_number(k: i64, x: i32) -> i64 {
        if k == 1 && x == 1 {
            return 1;
        }
        let mut l = 1;
        let mut h = i64::MAX;
        let k = k as i128;
        while l + 1 < h {
            let m = l + (h - l) / 2;
            if Solution::count(m, x) <= k {
                l = m;
            } else {
                h = m;
            }
        }
        l
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_maximum_number() {
        assert_eq!(Solution::find_maximum_number(19, 6), 50);
        assert_eq!(Solution::find_maximum_number(9, 1), 6);
        assert_eq!(Solution::find_maximum_number(7, 2), 9);
    }
}
