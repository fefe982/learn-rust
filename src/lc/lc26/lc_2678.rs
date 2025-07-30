// https://leetcode.com/problems/number-of-senior-citizens/
// 2678. Number of Senior Citizens
pub struct Solution;
impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .iter()
            .map(|x| {
                if x[11..13].parse::<i32>().unwrap() > 60 {
                    1
                } else {
                    0
                }
            })
            .sum::<i32>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_seniors() {
        assert_eq!(
            Solution::count_seniors(vec_str![
                "7868190130M7522",
                "5303914400F9211",
                "9273338290F4010"
            ]),
            2
        );
        assert_eq!(
            Solution::count_seniors(vec_str!["1313579440F2036", "2921522980M5644"]),
            0
        );
    }
}
