// https://leetcode.com/problems/total-distance-traveled/
// 2739. Total Distance Traveled
pub struct Solution;
impl Solution {
    pub fn distance_traveled(main_tank: i32, additional_tank: i32) -> i32 {
        let mut total = 0;
        let mut main_tank = main_tank;
        let mut additional_tank = additional_tank;
        while main_tank >= 5 {
            let d = main_tank / 5;
            main_tank = main_tank % 5;
            total += d * 5;
            let add = additional_tank.min(d);
            main_tank += add;
            additional_tank -= add;
        }
        (total + main_tank) * 10
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_distance_traveled() {
        assert_eq!(Solution::distance_traveled(5, 10), 60);
        assert_eq!(Solution::distance_traveled(1, 2), 10);
    }
}
