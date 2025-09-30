// https://leetcode.com/problems/number-of-days-between-two-dates/
// 1360. Number of Days Between Two Dates
pub struct Solution;
impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        let mut date1 = date1.split('-').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let mut date2 = date2.split('-').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        if date1 > date2 {
            date1.swap_with_slice(&mut date2);
        }
        let days = vec![0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let get_days = |y: i32, m: usize, d: i32| -> i32 {
            days[m - 1]
                + d
                + if m > 2 && (y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)) {
                    1
                } else {
                    0
                }
        };
        let day1 = get_days(date1[0], date1[1] as usize, date1[2]);
        let day2 = get_days(date2[0], date2[1] as usize, date2[2]);
        let get_leap = |y: i32| -> i32 { y / 4 - y / 100 + y / 400 };
        (date2[0] - date1[0]) * 365 + get_leap(date2[0] - 1) - get_leap(date1[0] - 1) + day2 - day1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn days_between_dates() {
        assert_eq!(
            Solution::days_between_dates("2008-03-21".to_string(), "2041-05-08".to_string()),
            12101
        );
        assert_eq!(
            Solution::days_between_dates("2019-06-29".to_string(), "2019-06-30".to_string()),
            1
        );
        assert_eq!(
            Solution::days_between_dates("2020-01-15".to_string(), "2019-12-31".to_string()),
            15
        );
    }
}
