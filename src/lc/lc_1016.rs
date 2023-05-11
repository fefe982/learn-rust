// https://leetcode.com/problems/binary-string-with-substrings-representing-1-to-n/
// 1016. Binary String With Substrings Representing 1 To N
pub struct Solution;
impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        let mut flag = vec![false; n as usize + 1];
        let s = s.as_bytes();
        for idx in 0..s.len() {
            if s[idx] == b'0' {
                continue;
            }
            let mut i = idx + 1;
            let mut cnt = 1;
            while cnt <= n {
                flag[cnt as usize] = true;
                if i >= s.len() {
                    break;
                }
                cnt = cnt * 2 + (s[i] - b'0') as i32;
                i += 1;
            }
        }
        for idx in 0..n {
            if !flag[idx as usize + 1] {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base_neg2() {
        assert_eq!(
            Solution::query_string(String::from("10010111100001110010"), 10),
            false
        );
        assert_eq!(Solution::query_string(String::from("0110"), 3), true);
        assert_eq!(Solution::query_string(String::from("0110"), 4), false);
    }
}
