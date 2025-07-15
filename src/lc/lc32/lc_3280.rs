// https://leetcode.com/problems/convert-date-to-binary/
// 3280. Convert Date to Binary
pub struct Solution;
impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        date.split('-')
            .map(|s| format!("{:b}", s.parse::<i32>().unwrap()))
            .collect::<Vec<_>>()
            .join("-")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn convert_date_to_binary() {
        assert_eq!(
            Solution::convert_date_to_binary("2080-02-29".to_string()),
            "100000100000-10-11101".to_string()
        );
        assert_eq!(
            Solution::convert_date_to_binary("1900-01-01".to_string()),
            "11101101100-1-1".to_string()
        );
    }
}
