// https://leetcode.com/problems/time-needed-to-inform-all-employees/
// 1376. Time Needed to Inform All Employees
pub struct Solution;
impl Solution {
    fn get_time(
        manager: &Vec<i32>,
        inform_time: &Vec<i32>,
        idx: usize,
        time: &mut Vec<i32>,
    ) -> i32 {
        if time[idx] > 0 {
            return time[idx];
        }
        let t = if manager[idx] < 0 {
            inform_time[idx]
        } else {
            inform_time[idx] + Self::get_time(manager, inform_time, manager[idx] as usize, time)
        };
        time[idx] = t;
        t
    }
    pub fn num_of_minutes(_n: i32, _head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut time = vec![0; inform_time.len()];
        for idx in 0..manager.len() {
            max = std::cmp::max(Self::get_time(&manager, &inform_time, idx, &mut time), max);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_of_minutes() {
        assert_eq!(Solution::num_of_minutes(1, 0, vec![-1], vec![0]), 0);
        assert_eq!(
            Solution::num_of_minutes(6, 2, vec![2, 2, -1, 2, 2, 2], vec![0, 0, 1, 0, 0, 0]),
            1
        );
    }
}
