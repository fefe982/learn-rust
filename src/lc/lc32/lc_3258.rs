// https://leetcode.com/problems/count-substrings-that-satisfy-k-constraint-i/
// 3258. Count Substrings That Satisfy K-Constraint I
pub struct Solution;
impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let mut ans = 0;
        let s = s.as_bytes();
        let mut cnt0 = 0;
        let mut cnt1 = 0;
        let mut i = 0;
        let mut j = 0;
        while j < s.len() {
            while j < s.len()
                && ((s[j] == b'0' && (cnt0 < k || cnt1 <= k)) || (s[j] == b'1' && (cnt0 <= k || cnt1 < k)))
            {
                if s[j] == b'0' {
                    cnt0 += 1;
                } else {
                    cnt1 += 1;
                }
                j += 1;
            }
            if j == s.len() {
                ans += (j - i + 1) as i32 * (j - i) as i32 / 2;
                break;
            }
            ans += (j - i) as i32;
            if s[i] == b'0' {
                cnt0 -= 1;
            } else {
                cnt1 -= 1;
            }
            i += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_k_constraint_substrings() {
        assert_eq!(Solution::count_k_constraint_substrings("000011".to_string(), 1), 18);
        assert_eq!(Solution::count_k_constraint_substrings("10101".to_string(), 1), 12);
        assert_eq!(Solution::count_k_constraint_substrings("1010101".to_string(), 2), 25);
        assert_eq!(Solution::count_k_constraint_substrings("11111".to_string(), 1), 15);
    }
}
