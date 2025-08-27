// https://leetcode.com/problems/distance-between-bus-stops/
// 1184. Distance Between Bus Stops
pub struct Solution;
impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let mut distance = distance;
        let mut sum = 0;
        for i in 0..distance.len() {
            let nsum = sum + distance[i];
            distance[i] = sum;
            sum = nsum;
        }
        let dist = (distance[destination as usize] - distance[start as usize]).abs();
        dist.min(sum - dist)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn distance_between_bus_stops() {
        assert_eq!(Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 1), 1);
        assert_eq!(Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 2), 3);
        assert_eq!(Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 3), 4);
    }
}
