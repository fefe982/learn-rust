// https://leetcode.com/problems/minimum-number-of-swaps-to-make-the-binary-string-alternating/
// 1864. Minimum Number of Swaps to Make the Binary String Alternating
pub struct Solution;
impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let count = s.chars().filter(|&c| c == '1').count();
        let n = s.len();

        if (n % 2 == 0 && count != n / 2) || (n % 2 == 1 && (count != n / 2 && count != n / 2 + 1)) {
            return -1;
        }

        let mismatch = |start: u8| -> i32 {
            let mut mismatch_count = 0;
            for (i, c) in s.bytes().enumerate() {
                let expected = if i % 2 == 0 {
                    start
                } else if start == b'0' {
                    b'1'
                } else {
                    b'0'
                };
                if c != expected {
                    mismatch_count += 1;
                }
            }
            mismatch_count / 2
        };

        if n % 2 == 0 {
            mismatch(b'0').min(mismatch(b'1'))
        } else if count == n / 2 + 1 {
            mismatch(b'1')
        } else {
            mismatch(b'0')
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_swaps() {
        assert_eq!(Solution::min_swaps("111000".to_string()), 1);
        assert_eq!(Solution::min_swaps("010".to_string()), 0);
        assert_eq!(Solution::min_swaps("1110".to_string()), -1);
    }
}
