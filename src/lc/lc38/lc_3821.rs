// https://leetcode.com/problems/find-nth-smallest-integer-with-k-one-bits/
// 3821. Find Nth Smallest Integer With K One Bits
pub struct Solution;
impl Solution {
    pub fn nth_smallest(n: i64, k: i32) -> i64 {
        let mut ans = 0;
        let mut k = k as i64;
        let mut n = n;
        while k > 0 {
            let mut b = 0;
            let mut last = 0;
            let mut comb = 1;
            while comb < n {
                last = comb;
                b += 1;
                comb = comb * (b + k) / b;
            }
            ans |= 1 << (b + k - 1);
            k -= 1;
            n -= last;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nth_smallest() {
        assert_eq!(Solution::nth_smallest(4, 2), 9);
        assert_eq!(Solution::nth_smallest(3, 1), 4);
    }
}
