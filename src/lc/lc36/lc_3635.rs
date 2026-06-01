// https://leetcode.cn/problems/earliest-finish-time-for-land-and-water-rides-ii/
// 3635. Earliest Finish Time for Land and Water Rides II
pub struct Solution;
impl Solution {
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        fn get_time(start_time: &[i32], duration: &[i32], time: i32) -> i32 {
            let mut end = i32::MAX;
            for (s, d) in start_time.iter().zip(duration.iter()) {
                end = end.min((*s).max(time) + *d);
            }
            end
        }
        get_time(
            &land_start_time,
            &land_duration,
            get_time(&water_start_time, &water_duration, 0),
        )
        .min(get_time(
            &water_start_time,
            &water_duration,
            get_time(&land_start_time, &land_duration, 0),
        ))
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
