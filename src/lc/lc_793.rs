// https://leetcode.com/problems/preimage-size-of-factorial-zeroes-function/
// 793. Preimage Size of Factorial Zeroes Function
pub struct Solution;
impl Solution {
    fn count(mut k: i32) -> i32 {
        let mut c = 0;
        while k > 0 {
            c += k;
            k /= 5;
        }
        c
    }
    pub fn preimage_size_fzf(k: i32) -> i32 {
        if Self::count(k) == k {
            return 5;
        }
        let mut min = 0;
        let mut max = k;
        while min + 1 < max {
            let mid = (min + max) / 2;
            let c = Self::count(mid);
            if c == k {
                return 5;
            }
            if c < k {
                min = mid;
            } else {
                max = mid;
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_preimage_size_fzf() {
        assert_eq!(Solution::preimage_size_fzf(0), 5);
        assert_eq!(Solution::preimage_size_fzf(5), 0);
        assert_eq!(Solution::preimage_size_fzf(3), 5);
    }
}
