// https://leetcode.com/problems/minimum-number-of-operations-to-convert-time/
// 2224. Minimum Number of Operations to Convert Time
pub struct Solution;
impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        fn to_minutes(s: &str) -> i32 {
            let mut iter = s.split(':');
            iter.next().unwrap().parse::<i32>().unwrap() * 60 + iter.next().unwrap().parse::<i32>().unwrap()
        }
        let mut ans = 0;
        let mut diff = to_minutes(&correct) - to_minutes(&current);
        for &d in &[60, 15, 5, 1] {
            ans += diff / d;
            diff %= d;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn convert_time() {
        assert_eq!(Solution::convert_time("02:30".to_string(), "04:35".to_string()), 3);
        assert_eq!(Solution::convert_time("11:00".to_string(), "11:01".to_string()), 1);
    }
}
