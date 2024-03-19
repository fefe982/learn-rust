// https://leetcode.com/problems/task-scheduler/
// 621. Task Scheduler
pub struct Solution;
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut freq = vec![0; 26];
        let ntask = tasks.len() as i32;
        for c in tasks {
            freq[(c as u8 - b'A') as usize] += 1;
        }
        freq.sort_unstable();
        let max_freq = freq[25];
        let mut idle_slots = (max_freq - 1) * n;
        for i in (0..25).rev() {
            idle_slots -= std::cmp::min(freq[i], max_freq - 1);
        }
        if idle_slots < 0 {
            ntask
        } else {
            ntask + idle_slots
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_least_interval() {
        assert_eq!(Solution::least_interval(vec_chr!["A", "A", "A", "B", "B", "B"], 2), 8);
        assert_eq!(Solution::least_interval(vec_chr!["A", "C", "A", "B", "D", "B"], 1), 6);
        assert_eq!(Solution::least_interval(vec_chr!["A", "A", "A", "B", "B", "B"], 3), 10);
    }
}
