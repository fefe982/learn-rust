// https://leetcode.com/problems/latest-time-by-replacing-hidden-digits/
// 1736. Latest Time by Replacing Hidden Digits
pub struct Solution;
impl Solution {
    pub fn maximum_time(time: String) -> String {
        let mut chars = time.chars().collect::<Vec<char>>();
        if chars[0] == '?' {
            if chars[1] == '?' {
                chars[0] = '2';
                chars[1] = '3';
            } else if chars[1] < '4' {
                chars[0] = '2';
            } else {
                chars[0] = '1';
            }
        } else if chars[1] == '?' {
            if chars[0] == '2' {
                chars[1] = '3';
            } else {
                chars[1] = '9';
            }
        }
        if chars[3] == '?' {
            chars[3] = '5';
        }
        if chars[4] == '?' {
            chars[4] = '9';
        }
        chars.into_iter().collect::<String>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_time() {
        assert_eq!(Solution::maximum_time("2?:?0".to_string()), "23:50");
        assert_eq!(Solution::maximum_time("0?:3?".to_string()), "09:39");
        assert_eq!(Solution::maximum_time("1?:22".to_string()), "19:22");
    }
}
