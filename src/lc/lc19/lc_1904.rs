// https://leetcode.com/problems/the-number-of-full-rounds-you-have-played/
// 1904. The Number of Full Rounds You Have Played
pub struct Solution;
impl Solution {
    pub fn number_of_rounds(login_time: String, logout_time: String) -> i32 {
        let parse_time = |s: String| {
            let mut iter = s.split(':').map(|x| x.parse::<i32>().unwrap());
            iter.next().unwrap() * 60 + iter.next().unwrap()
        };
        let mut start = parse_time(login_time);
        let mut end = parse_time(logout_time);
        if end < start {
            end += 24 * 60;
        }
        if start % 15 != 0 {
            start += 15 - start % 15;
        }
        (end - start) / 15
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_rounds() {
        assert_eq!(Solution::number_of_rounds("09:31".to_string(), "10:14".to_string()), 1);
        assert_eq!(Solution::number_of_rounds("21:30".to_string(), "03:00".to_string()), 22);
    }
}
