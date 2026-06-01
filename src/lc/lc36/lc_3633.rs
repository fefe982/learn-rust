// https://leetcode.com/problems/earliest-finish-time-for-land-and-water-rides-i/
// 3633. Earliest Finish Time for Land and Water Rides I
pub struct Solution;
impl Solution {
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        super::lc_3635::Solution::earliest_finish_time(land_start_time, land_duration, water_start_time, water_duration)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_earliest_finish_time() {
        assert_eq!(
            Solution::earliest_finish_time(vec![2, 8], vec![4, 1], vec![6], vec![3]),
            9
        );
        assert_eq!(Solution::earliest_finish_time(vec![5], vec![3], vec![1], vec![10]), 14);
    }
}
