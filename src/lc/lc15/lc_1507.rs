// https://leetcode.com/problems/reformat-date/
// 1507. Reformat Date
pub struct Solution;
impl Solution {
    pub fn reformat_date(date: String) -> String {
        let date = date.split_ascii_whitespace().collect::<Vec<_>>();
        let mut ans = String::new();
        ans.push_str(date[2]);
        ans.push('-');
        ans.push_str(match date[1] {
            "Jan" => "01",
            "Feb" => "02",
            "Mar" => "03",
            "Apr" => "04",
            "May" => "05",
            "Jun" => "06",
            "Jul" => "07",
            "Aug" => "08",
            "Sep" => "09",
            "Oct" => "10",
            "Nov" => "11",
            "Dec" => "12",
            _ => unreachable!(),
        });
        ans.push('-');
        if date[0].len() == 3 {
            ans.push('0');
            ans.push(date[0].chars().nth(0).unwrap());
        } else {
            ans.push_str(date[0].chars().take(2).collect::<String>().as_str());
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reformat_date() {
        assert_eq!(
            Solution::reformat_date("20th Oct 2052".to_string()),
            "2052-10-20".to_string()
        );
        assert_eq!(
            Solution::reformat_date("6th Jun 1933".to_string()),
            "1933-06-06".to_string()
        );
        assert_eq!(
            Solution::reformat_date("26th May 1960".to_string()),
            "1960-05-26".to_string()
        );
    }
}
