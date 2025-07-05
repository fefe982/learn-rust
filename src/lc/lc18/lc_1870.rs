// https://leetcode.com/problems/minimum-speed-to-arrive-on-time/
// 1870. Minimum Speed to Arrive on Time
pub struct Solution;
impl Solution {
    fn check_speed(dist: &Vec<i32>, speed: i32, hour: f64) -> bool {
        dist.iter()
            .take(dist.len() - 1)
            .fold(0.0, |acc, &d| acc + (d as f64 / speed as f64 - 1e-9).ceil())
            + (*dist.last().unwrap() as f64 / speed as f64)
            < hour + 1e-9
    }
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        let mut low = 1;
        if Self::check_speed(&dist, low, hour) {
            return low;
        }
        if hour <= dist.len() as f64 - 1.0 {
            return -1;
        }
        let mut high = (*dist.iter().max().unwrap())
            .max((*dist.last().unwrap() as f64 / (hour - dist.len() as f64 + 1.0)).ceil() as i32);
        if !Self::check_speed(&dist, high, hour) {
            return -1;
        }
        while low + 1 < high {
            let mid = (low + high) / 2;
            if Self::check_speed(&dist, mid, hour) {
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
    fn min_speed_on_time() {
        assert_eq!(
            Solution::min_speed_on_time(vec![1, 1, 100000], 2.01),
            10000000
        );
        assert_eq!(Solution::min_speed_on_time(vec![1, 3, 2], 6.0), 1);
        assert_eq!(Solution::min_speed_on_time(vec![1, 3, 2], 2.7), 3);
        assert_eq!(Solution::min_speed_on_time(vec![1, 3, 2], 1.9), -1)
    }
}
