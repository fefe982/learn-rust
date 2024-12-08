// https://leetcode.com/problems/two-best-non-overlapping-events/
// 2054. Two Best Non-Overlapping Events
pub struct Solution;
impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events;
        events.sort_unstable();
        let mut m = vec![0; events.len()];
        m[events.len() - 1] = events[events.len() - 1][2];
        for i in (0..events.len() - 1).rev() {
            m[i] = m[i + 1].max(events[i][2]);
        }
        let mut max = 0;
        for i in 0..events.len() {
            let p = events.partition_point(|x| x[0] <= events[i][1]);
            if p < events.len() {
                max = max.max(events[i][2] + m[p]);
            } else {
                max = max.max(events[i][2]);
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_two_events() {
        assert_eq!(Solution::max_two_events(vec_vec![[1, 3, 2], [4, 5, 2], [2, 4, 3]]), 4);
        assert_eq!(Solution::max_two_events(vec_vec![[1, 3, 2], [4, 5, 2], [1, 5, 5]]), 5);
        assert_eq!(Solution::max_two_events(vec_vec![[1, 5, 3], [1, 5, 1], [6, 6, 5]]), 8);
    }
}
