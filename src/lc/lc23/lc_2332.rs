// https://leetcode.cn/problems/the-latest-time-to-catch-a-bus/
// 2332. The Latest Time to Catch a Bus
pub struct Solution;
impl Solution {
    pub fn latest_time_catch_the_bus(buses: Vec<i32>, passengers: Vec<i32>, capacity: i32) -> i32 {
        let mut buses = buses;
        let mut passengers = passengers;
        buses.sort();
        passengers.sort();
        let mut gap = (passengers[0] - 1).min(buses[0]);
        let mut ibus = 0;
        let mut ipassenger = 0;
        while ibus < buses.len() {
            let mut npassenger = 0;
            while npassenger < capacity {
                if ipassenger < passengers.len() && passengers[ipassenger] <= buses[ibus] {
                    if ipassenger == 0 || passengers[ipassenger] - passengers[ipassenger - 1] > 1 {
                        gap = passengers[ipassenger] - 1;
                    }
                    ipassenger += 1;
                    npassenger += 1;
                } else {
                    break;
                }
            }
            if npassenger < capacity {
                if ipassenger == 0 || passengers[ipassenger - 1] < buses[ibus] {
                    if ipassenger == passengers.len() {
                        gap = buses[ibus];
                    } else if ipassenger == 0 || passengers[ipassenger] - passengers[ipassenger - 1] > 1 {
                        gap = (passengers[ipassenger] - 1).min(buses[ibus]);
                    }
                }
            }
            ibus += 1;
        }
        gap
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_latest_time_catch_the_bus() {
        assert_eq!(
            Solution::latest_time_catch_the_bus(vec![5, 15], vec![11, 12, 13, 14, 15], 2),
            10
        );
        assert_eq!(Solution::latest_time_catch_the_bus(vec![2], vec![2], 2), 1);
        assert_eq!(Solution::latest_time_catch_the_bus(vec![5], vec![7, 8], 1), 5);
        assert_eq!(
            Solution::latest_time_catch_the_bus(vec![10, 20], vec![2, 17, 18, 19], 2),
            16
        );
        assert_eq!(
            Solution::latest_time_catch_the_bus(vec![20, 30, 10], vec![19, 13, 26, 4, 25, 11, 21], 2),
            20
        );
    }
}
