// https://leetcode.com/problems/calculate-delayed-arrival-time/
// 2651. Calculate Delayed Arrival Time
pub struct Solution;
impl Solution {
    pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
        (arrival_time + delayed_time) % 24
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_delayed_arrival_time() {
        assert_eq!(Solution::find_delayed_arrival_time(15, 5), 20);
        assert_eq!(Solution::find_delayed_arrival_time(13, 11), 0);
    }
}
