// https://leetcode.com/problems/splitting-a-string-into-descending-consecutive-values/
// 1849. Splitting a String Into Descending Consecutive Values
pub struct Solution;
impl Solution {
    pub fn split_string(s: String) -> bool {
        fn dfs(bytes: &[u8], idx: usize, prev: u128) -> bool {
            if idx == bytes.len() {
                return true;
            }
            if prev == 0 {
                return false;
            }

            let mut cur = 0_u128;
            for i in idx..bytes.len() {
                cur = cur * 10 + u128::from(bytes[i] - b'0');

                if cur + 1 == prev && dfs(bytes, i + 1, cur) {
                    return true;
                }
                if cur >= prev {
                    break;
                }
            }
            false
        }

        let bytes = s.as_bytes();
        let n = bytes.len();
        if n < 2 {
            return false;
        }

        let mut first = 0_u128;
        for i in 0..(n - 1) {
            first = first * 10 + u128::from(bytes[i] - b'0');
            if dfs(bytes, i + 1, first) {
                return true;
            }
        }

        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn split_string() {
        assert_eq!(Solution::split_string("1234".to_string()), false);
        assert_eq!(Solution::split_string("050043".to_string()), true);
        assert_eq!(Solution::split_string("9080701".to_string()), false);
    }
}
