// https://leetcode.com/problems/day-of-the-week/
// 1185. Day of the Week
pub struct Solution;
impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let base = 5;
        let md = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let mut d = ((year - 1971) * 365) % 7 + (year - 1) / 4 - 1970 / 4 - (year - 1) / 100 + 19 + (year - 1) / 400
            - 1970 / 400;
        d += md[month as usize - 1];
        if month > 2 && year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
            d += 1;
        }
        match (d + day - 1 + base) % 7 {
            0 => "Sunday",
            1 => "Monday",
            2 => "Tuesday",
            3 => "Wednesday",
            4 => "Thursday",
            5 => "Friday",
            6 => "Saturday",
            _ => "",
        }
        .to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_of_the_week() {
        assert_eq!(Solution::day_of_the_week(1, 1, 1971), "Friday");
        assert_eq!(Solution::day_of_the_week(31, 8, 2019), "Saturday");
        assert_eq!(Solution::day_of_the_week(18, 7, 1999), "Sunday");
        assert_eq!(Solution::day_of_the_week(15, 8, 1993), "Sunday");
    }
}
