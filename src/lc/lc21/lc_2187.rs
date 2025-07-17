// https://leetcode.com/problems/minimum-time-to-complete-trips/
// 2187. Minimum Time to Complete Trips
pub struct Solution;
impl Solution {
    fn check(time: &Vec<i32>, total_trips: i32, total_time: i64) -> bool {
        let mut trip_cnt = 0_i64;
        for &t in time.iter() {
            trip_cnt += total_time / (t as i64);
        }
        trip_cnt >= total_trips as i64
    }
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let mut min_time = i32::MAX;
        for &t in time.iter() {
            if t < min_time {
                min_time = t;
            }
        }
        let mut high = min_time as i64 * total_trips as i64;
        let mut low = 0_i64;
        while low < high {
            if low + 1 == high {
                return high;
            }
            let mid = (low + high) / 2;
            if Self::check(&time, total_trips, mid) {
                high = mid;
            } else {
                low = mid;
            }
            println!("{}, {}", low, high);
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_time() {
        assert_eq!(Solution::minimum_time(vec![1, 2, 3], 5), 3);
        assert_eq!(Solution::minimum_time(vec![2], 1), 2);
    }
}
