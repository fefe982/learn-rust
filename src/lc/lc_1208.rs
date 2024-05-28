// https://leetcode.com/problems/get-equal-substrings-within-budget/
// 1208. Get Equal Substrings Within Budget
pub struct Solution;
impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut i = 0;
        let mut j = 0;
        let mut cost = 0;
        let mut max_length = 0;
        loop {
            while j < t.len() && cost + (t[j] as i32 - s[j] as i32).abs() <= max_cost {
                cost += (t[j] as i32 - s[j] as i32).abs();
                j += 1;
            }
            max_length = max_length.max(j - i);
            if j >= t.len() {
                break;
            }
            cost += (t[j] as i32 - s[j] as i32).abs();
            j += 1;
            while i < j && cost > max_cost {
                cost -= (t[i] as i32 - s[i] as i32).abs();
                i += 1;
            }
        }
        max_length as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_equal_substring() {
        assert_eq!(Solution::equal_substring("abcd".to_string(), "bcdf".to_string(), 3), 3);
        assert_eq!(Solution::equal_substring("abcd".to_string(), "cdef".to_string(), 3), 1);
        assert_eq!(Solution::equal_substring("abcd".to_string(), "acde".to_string(), 0), 1);
    }
}
