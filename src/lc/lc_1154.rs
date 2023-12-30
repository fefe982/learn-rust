// https://leetcode.com/problems/day-of-the-year/
// 1154. Day of the Year
pub struct Solution;
impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let date = date.split('-').collect::<Vec<_>>();
        let y = date[0].parse::<i32>().unwrap();
        let m = date[1].parse::<usize>().unwrap();
        let d = date[2].parse::<i32>().unwrap();
        let days = vec![0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        days[m - 1]
            + d
            + if m > 2 && (y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)) {
                1
            } else {
                0
            }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_of_year() {
        assert_eq!(Solution::day_of_year(String::from("2019-01-09")), 9);
        assert_eq!(Solution::day_of_year(String::from("2019-02-10")), 41);
    }
}
