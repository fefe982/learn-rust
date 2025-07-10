// https://leetcode.com/problems/find-products-of-elements-of-big-array/
// 3145. Find Products of Elements of Big Array
pub struct Solution;
impl Solution {
    fn count_n(n: i64) -> i64 {
        let mut n = n;
        let mut res = 0;
        let mut prefix = 0;
        while n > 0 {
            let bit = 63 - n.leading_zeros() as i64;
            res += (1 << bit) * bit / 2 + prefix * (1 << bit);
            prefix += 1;
            n &= !(1 << bit);
        }
        res + prefix
    }
    fn sum_n(n: i64) -> i64 {
        let mut n = n;
        let mut res = 0;
        let mut prefix = 0;
        while n > 0 {
            let bit = 63 - n.leading_zeros() as i64;
            res += (1 << bit) * (bit - 1) * bit / 4 + prefix * (1 << bit);
            prefix += bit;
            n &= !(1 << bit);
        }
        res + prefix
    }
    fn q(n: i64) -> i64 {
        let mut l = 1i64;
        let mut r = 2i64 << 45;
        while l + 1 < r {
            let m = l + (r - l) / 2;
            match Self::count_n(m).cmp(&n) {
                std::cmp::Ordering::Equal => return Self::sum_n(m),
                std::cmp::Ordering::Less => l = m,
                std::cmp::Ordering::Greater => r = m,
            }
        }
        let mut res = Self::sum_n(l);
        for _ in 0..n - Self::count_n(l) {
            let nz = r.trailing_zeros() as i64;
            res += nz;
            r &= !(1 << nz);
        }
        res
    }
    fn pow_mod(n: i64, m: i64) -> i32 {
        if m == 1 {
            return 0;
        }
        let mut pow = 2;
        let mut n = n;
        let mut res = 1;
        while n > 0 {
            if n & 1 > 0 {
                res = res * pow % m;
            }
            pow = pow * pow % m;
            n >>= 1;
        }
        res as i32
    }
    pub fn find_products_of_elements(queries: Vec<Vec<i64>>) -> Vec<i32> {
        queries
            .into_iter()
            .map(|q| {
                if q[2] == 1 {
                    0
                } else {
                    Self::pow_mod(Self::q(q[1] + 1) - Self::q(q[0]), q[2])
                }
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_products_of_elements() {
        assert_eq!(Solution::find_products_of_elements(vec_vec![[9, 9, 1]]), [0]);
        assert_eq!(Solution::find_products_of_elements(vec_vec![[0, 10, 6]]), [4]);
        assert_eq!(Solution::find_products_of_elements(vec_vec![[1, 3, 7]]), [4]);
        assert_eq!(
            Solution::find_products_of_elements(vec_vec![[2, 5, 3], [7, 7, 4]]),
            [2, 2],
        );
    }
}
