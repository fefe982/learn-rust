// https://leetcode.com/problems/latest-time-you-can-obtain-after-replacing-characters/
// 3114. Latest Time by Replacing Hidden Digits
pub struct Solution;
impl Solution {
    pub fn find_latest_time(s: String) -> String {
        let mut sc = s.chars().collect::<Vec<char>>();
        if sc[0] == '?' && sc[1] == '?' {
            sc[0] = '1';
            sc[1] = '1';
        } else if sc[0] == '?' {
            if sc[1] == '0' || sc[1] == '1' {
                sc[0] = '1';
            } else {
                sc[0] = '0';
            }
        } else if sc[1] == '?' {
            if sc[0] == '1' {
                sc[1] = '1';
            } else {
                sc[1] = '9';
            }
        }
        if sc[3] == '?' {
            sc[3] = '5';
        }
        if sc[4] == '?' {
            sc[4] = '9';
        }
        sc.into_iter().collect::<String>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_latest_time() {
        assert_eq!(Solution::find_latest_time("1?:?4".to_string()), "11:54".to_string());
        assert_eq!(Solution::find_latest_time("0?:5?".to_string()), "09:59".to_string());
    }
}
