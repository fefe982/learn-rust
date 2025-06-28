// https://leetcode.com/problems/minimum-number-of-days-to-make-m-bouquets/
// 1482. Minimum Number of Days to Make m Bouquets
pub struct Solution;
impl Solution {
    fn check(boom_day: &Vec<i32>, m: i32, k: i32, day: i32) -> bool {
        let mut cnt = 0;
        let mut flower = 0;
        for &b in boom_day.iter() {
            if b <= day {
                flower += 1;
            } else {
                cnt += flower / k;
                flower = 0;
            }
        }
        cnt + flower / k >= m
    }
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        if m as i64 * k as i64 > bloom_day.len() as i64 {
            return -1;
        }
        if Self::check(&bloom_day, m, k, 1) {
            return 1;
        }
        let mut low = 1;
        let mut high = *bloom_day.iter().max().unwrap();
        while low + 1 < high {
            let mid = (low + high) / 2;
            if Self::check(&bloom_day, m, k, mid) {
                high = mid;
            } else {
                low = mid;
            }
        }
        high
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_days() {
        assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 1), 3);
        assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 2), -1);
        assert_eq!(Solution::min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3), 12);
    }
}
