// https://leetcode.com/problems/decode-ways/
// 91. Decode Ways
pub struct Solution;
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut f1 = 1;
        let mut f2 = 0;
        let mut last = '9';
        for c in s.chars() {
            let f = match c {
                '0' => {
                    if last == '1' || last == '2' {
                        f2
                    } else {
                        0
                    }
                }
                '1'..='6' => {
                    if last == '1' || last == '2' {
                        f1 + f2
                    } else {
                        f1
                    }
                }
                _ => {
                    if last == '1' {
                        f1 + f2
                    } else {
                        f1
                    }
                }
            };
            f2 = f1;
            f1 = f;
            last = c;
        }
        f1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_decodings() {
        assert_eq!(Solution::num_decodings("12".to_string()), 2);
        assert_eq!(Solution::num_decodings("226".to_string()), 3);
        assert_eq!(Solution::num_decodings("06".to_string()), 0);
    }
}
