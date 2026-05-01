// https://leetcode.com/problems/maximum-product-of-the-length-of-two-palindromic-subsequences/
// 2002. Maximum Product of the Length of Two Palindromic Subsequences
pub struct Solution;
impl Solution {
    pub fn max_product(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let total = 1usize << n;
        let mut pal_len = vec![0i32; total];

        for mask in 1..total {
            if Self::is_palindrome_mask(bytes, mask) {
                pal_len[mask] = mask.count_ones() as i32;
            }
        }

        let mut ans = 0;
        for m1 in 1..total {
            if pal_len[m1] == 0 {
                continue;
            }
            for m2 in (m1 + 1)..total {
                if (m1 & m2) == 0 && pal_len[m2] > 0 {
                    ans = ans.max(pal_len[m1] * pal_len[m2]);
                }
            }
        }
        ans
    }

    fn is_palindrome_mask(bytes: &[u8], mask: usize) -> bool {
        let n = bytes.len();
        let mut l = 0usize;
        let mut r = n - 1;

        while l < r {
            while l < r && ((mask >> l) & 1) == 0 {
                l += 1;
            }
            while l < r && ((mask >> r) & 1) == 0 {
                r -= 1;
            }
            if l < r {
                if bytes[l] != bytes[r] {
                    return false;
                }
                l += 1;
                r -= 1;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_product() {
        assert_eq!(Solution::max_product("leetcodecom".to_string()), 9);
        assert_eq!(Solution::max_product("bb".to_string()), 1);
        assert_eq!(Solution::max_product("accbcaxxcxx".to_string()), 25);
    }
}
