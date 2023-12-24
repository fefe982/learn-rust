// https://leetcode.com/problems/minimum-number-of-refueling-stops/
// 871. Minimum Number of Refueling Stops
pub struct Solution;
impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut q = std::collections::BinaryHeap::new();
        let mut fuel = start_fuel;
        let mut cnt = 0;
        for v in stations.into_iter().chain(vec![vec![target, 0]].into_iter()) {
            let s_pos = v[0];
            let s_fuel = v[1];
            while s_pos > fuel {
                if let Some(f) = q.pop() {
                    fuel += f;
                    cnt += 1;
                } else {
                    return -1;
                }
            }
            q.push(s_fuel);
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_fuel_stops() {
        assert_eq!(Solution::min_refuel_stops(1, 1, vec![]), 0);
        assert_eq!(Solution::min_refuel_stops(100, 1, vec_vec![[10, 100]]), -1);
        assert_eq!(
            Solution::min_refuel_stops(100, 10, vec_vec![[10, 60], [20, 30], [30, 30], [60, 40]]),
            2
        );
    }
}
