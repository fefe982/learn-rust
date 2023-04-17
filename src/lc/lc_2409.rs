// https://leetcode.com/problems/count-days-spent-together/
// 2409. Count Days Spent Together
pub struct Solution;
impl Solution {
    fn to_int(n: &[u8]) -> i32 {
        let mut r = 0;
        n.iter().for_each(|&d| r = r * 10 + (d - b'0') as i32);
        r
    }
    fn parse_date(date: String) -> i32 {
        let date = date.as_bytes();
        let dates = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let month = Self::to_int(&date[0..2]);
        let date = Self::to_int(&date[3..5]);
        dates[month as usize - 1] + date - 1
    }
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        std::cmp::max(
            0,
            std::cmp::min(Self::parse_date(leave_alice), Self::parse_date(leave_bob))
                - std::cmp::max(Self::parse_date(arrive_alice), Self::parse_date(arrive_bob))
                + 1,
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_days_together() {
        assert_eq!(
            Solution::count_days_together(
                String::from("08-15"),
                String::from("08-18"),
                String::from("08-16"),
                String::from("08-19")
            ),
            3
        );
        assert_eq!(
            Solution::count_days_together(
                String::from("10-01"),
                String::from("10-31"),
                String::from("11-01"),
                String::from("12-31")
            ),
            0
        );
    }
}
